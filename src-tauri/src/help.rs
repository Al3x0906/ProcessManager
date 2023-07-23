//use windows::{core::Array, Win32::UI::WindowsAndMessaging::*};
use std::mem;

use serde::ser::{Serialize, SerializeStruct, Serializer};
//use windows_sys::Win32::Foundation::FILETIME;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Foundation::HMODULE;
use windows_sys::Win32::System::ProcessStatus::EnumProcesses;
use windows_sys::Win32::System::ProcessStatus::GetModuleBaseNameW;
use windows_sys::Win32::System::ProcessStatus::GetProcessMemoryInfo;
use windows_sys::Win32::System::ProcessStatus::PROCESS_MEMORY_COUNTERS;
use windows_sys::Win32::System::Threading::*;
type DWORD = u32;

pub struct Process {
    pid: u32,
    handle: HANDLE,
    name: String,
    mem_usage: f64,
    //cpu_usage: u64,
    //cpu_precentage: f64,
}
impl Serialize for Process {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Process", 3)?;
        s.serialize_field("pid", &self.pid)?;
        s.serialize_field("handle", &self.handle)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("mem_usage", &self.mem_usage)?;
        s.end()
    }
}

fn get_process_name(pid: u32) -> String {
    unsafe {
        let process = OpenProcess(PROCESS_ALL_ACCESS, 1, pid);
        let h_module: HMODULE = mem::zeroed();
        let mut buffer: [u16; 1024] = mem::zeroed();

        if GetModuleBaseNameW(process, h_module, buffer.as_mut_ptr(), buffer.len() as u32) > 0 {
            let process_name = String::from_utf16_lossy(&buffer);
            let name = process_name.trim_matches('\0');
            return String::from(name);
            //println!("{:?}", name)
        } else {
            return String::from("");
        }
    }
}

fn get_process_mem_usage(handle: HANDLE) -> f64 {
    let mut buffer: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
    let process_mem_info = &mut buffer as *mut PROCESS_MEMORY_COUNTERS;
    let cb = 1024u32;
    let res = unsafe { GetProcessMemoryInfo(handle, process_mem_info, cb) != 0 };

    if res {
        let working_set_size = (buffer.WorkingSetSize / 1024) as f64;
        let quota_paged_pool_usage = (buffer.QuotaPagedPoolUsage / 1024) as f64;
        let quota_non_paged_pool_usage = (buffer.QuotaNonPagedPoolUsage / 1024) as f64;

        let memory_used = working_set_size - (quota_paged_pool_usage + quota_non_paged_pool_usage);

        // println!("Memory Used: {} bytes", memory_used);
        return memory_used;
    } else {
        return -1 as f64;
    }
}
fn get_process(pid: u32) -> HANDLE {
    let result: HANDLE;
    unsafe {
        result = OpenProcess(1024u32, 1, pid);
    }
    //println!("{:?}", result);
    result
}

#[tauri::command]
pub fn processes() -> Vec<Process> {
    let mut process_ids: [DWORD; 1024] = [0; 1024];

    let mut needed: DWORD = 0;

    let _res = unsafe {
        EnumProcesses(
            process_ids.as_mut_ptr(),
            process_ids.len() as u32,
            &mut needed,
        )
    };

    let num_of_process = needed as u32 / mem::size_of::<u32>() as u32;

    let mut processes: Vec<Process> = Vec::new();
    for i in 0..num_of_process {
        let process_id = process_ids[i as usize];
        let handle: HANDLE = get_process(process_id.clone());
        let name = get_process_name(process_id);
        let mem_usage = get_process_mem_usage(handle);
        // get_process_cpu_usage(handle);

        let process1 = Process {
            pid: process_id,
            handle: handle,
            name: name,
            mem_usage: mem_usage,
        };
        if process1.mem_usage==-1f64 || process1.handle==0 || process1.name=="" {
            continue;
        }
        processes.push(process1);
    }
    return processes;
}
