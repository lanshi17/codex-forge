use crate::token::*;
use crate::winutil::to_wide;
use anyhow::{Result, ensure};
use std::ptr;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Security::*;
use windows_sys::Win32::Security::Authorization::*;
use windows_sys::Win32::System::StationsAndDesktops::*;
use windows_sys::Win32::System::Threading::*;
use windows_sys::Win32::System::JobObjects::*;
use windows_sys::Win32::System::Pipes::CreatePipe;

unsafe fn describe(handle: HANDLE, label: &str) -> Result<()> {
    let mut descriptor = ptr::null_mut();
    ensure!(unsafe { GetSecurityInfo(handle, SE_WINDOW_OBJECT, DACL_SECURITY_INFORMATION, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), &mut descriptor) } == 0);
    let mut sddl = ptr::null_mut();
    ensure!(unsafe { ConvertSecurityDescriptorToStringSecurityDescriptorW(descriptor, 1, DACL_SECURITY_INFORMATION, &mut sddl, ptr::null_mut()) } != 0);
    let mut length = 0;
    while unsafe { *sddl.add(length) } != 0 { length += 1; }
    eprintln!("{label}: {}", String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(sddl, length) }));
    unsafe { LocalFree(sddl.cast()); LocalFree(descriptor); }
    Ok(())
}

#[test]
fn startup_probe() -> Result<()> {
    unsafe {
        let base = get_current_token_for_restriction()?;
        let cap = LocalSid::from_string("S-1-5-21-10-20-30-40")?;
        let restricted = create_readonly_token_with_caps_from(base, &[cap.as_ptr()])?;
        describe(GetProcessWindowStation(), "station")?;
        let mut logon = get_logon_sid_bytes(base)?;
        let mut sid_text = ptr::null_mut();
        ensure!(ConvertSidToStringSidW(logon.as_mut_ptr().cast(), &mut sid_text) != 0);
        let mut length = 0;
        while *sid_text.add(length) != 0 { length += 1; }
        let logon_text = String::from_utf16_lossy(std::slice::from_raw_parts(sid_text, length));
        LocalFree(sid_text.cast());
        let mut descriptor = ptr::null_mut();
        ensure!(ConvertStringSecurityDescriptorToSecurityDescriptorW(to_wide(format!("D:P(A;;GA;;;{logon_text})")).as_ptr(), 1, &mut descriptor, ptr::null_mut()) != 0);
        let attributes = SECURITY_ATTRIBUTES { nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32, lpSecurityDescriptor: descriptor, bInheritHandle: 0 };
        let name = to_wide("CodexStartupProbe");
        let desktop = CreateDesktopW(name.as_ptr(), ptr::null(), ptr::null_mut(), 0, 0xF01FF, &attributes);
        LocalFree(descriptor);
        ensure!(!desktop.is_null(), "create desktop: {}", GetLastError());
        describe(desktop, "desktop")?;
        for use_private in [false, true] {
            for environment in ["inherit", "systemroot", "empty"] {
          for containment in [false, true] {
           for pipes in [false, true] {
                for token in [base, restricted] {
                    let executable = std::env::current_exe()?;
                    let mut command = to_wide(format!("\"{}\" --list", executable.display()));
                    let job = CreateJobObjectW(ptr::null(), ptr::null());
                    ensure!(!job.is_null());
                    let mut limits: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
                    limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE | JOB_OBJECT_LIMIT_BREAKAWAY_OK;
                    ensure!(SetInformationJobObject(job, JobObjectExtendedLimitInformation, (&limits as *const JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(), std::mem::size_of_val(&limits) as u32) != 0);
                    let mut attrs = crate::proc_thread_attr::ProcThreadAttributeList::new(u32::from(containment) + u32::from(pipes) + 1)?;
                    if containment { attrs.set_job(job)?; }
                    let mut startup: STARTUPINFOEXW = std::mem::zeroed();
                    startup.StartupInfo.cb = std::mem::size_of::<STARTUPINFOEXW>() as u32;
                    if use_private { startup.StartupInfo.lpDesktop = name.as_ptr().cast_mut(); }
                    let mut handles = vec![];
                    if pipes {
                        let (mut stdin_read, mut stdin_write, mut stdout_read, mut stdout_write) = (ptr::null_mut(),ptr::null_mut(),ptr::null_mut(),ptr::null_mut());
                        ensure!(CreatePipe(&mut stdin_read, &mut stdin_write, ptr::null(), 0) != 0);
                        ensure!(CreatePipe(&mut stdout_read, &mut stdout_write, ptr::null(), 0) != 0);
                        for h in [stdin_read, stdout_write] { ensure!(SetHandleInformation(h, HANDLE_FLAG_INHERIT, HANDLE_FLAG_INHERIT) != 0); }
                        startup.StartupInfo.dwFlags = STARTF_USESTDHANDLES;
                        startup.StartupInfo.hStdInput = stdin_read;
                        startup.StartupInfo.hStdOutput = stdout_write;
                        startup.StartupInfo.hStdError = stdout_write;
                        attrs.set_handle_list(vec![stdin_read,stdout_write])?;
                        handles.extend([stdin_read,stdin_write,stdout_read,stdout_write]);
                    }
                    startup.lpAttributeList = attrs.as_mut_ptr();
                    let mut info = std::mem::zeroed();
                    let mut env = if environment == "empty" { vec![0u16; 2] } else { to_wide(format!("SystemRoot={}\0", std::env::var("SystemRoot")?)) };
                    let environment_ptr = match environment {
                        "inherit" => ptr::null_mut(),
                        "systemroot" | "empty" => env.as_mut_ptr().cast(),
                        _ => unreachable!(),
                    };
                    ensure!(CreateProcessAsUserW(token, ptr::null(), command.as_mut_ptr(), ptr::null(), ptr::null(), i32::from(pipes), CREATE_NO_WINDOW | CREATE_UNICODE_ENVIRONMENT | EXTENDED_STARTUPINFO_PRESENT, environment_ptr, ptr::null(), &startup.StartupInfo, &mut info) != 0, "spawn: {}", GetLastError());
                    let wait = WaitForSingleObject(info.hProcess, 10000);
                    let mut exit = 0;
                    GetExitCodeProcess(info.hProcess, &mut exit);
                    if wait != WAIT_OBJECT_0 { TerminateProcess(info.hProcess, 99); }
                    eprintln!("startup job={containment} pipes={pipes} private={use_private} env={environment} restricted={} wait={wait} exit={exit:#x}", token == restricted);
                    CloseHandle(info.hThread); CloseHandle(info.hProcess); CloseHandle(job);
                    for h in handles { CloseHandle(h); }
                }
            }
        }
          }
         }
        CloseDesktop(desktop); CloseHandle(restricted); CloseHandle(base);
    }
    Ok(())
}
