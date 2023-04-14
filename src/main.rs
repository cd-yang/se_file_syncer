// use windows::{Guid, Interface, IntoParam, RuntimeName, Win32};
use windows::{
    core::*, Win32::Networking::BackgroundIntelligentTransferService::*, Win32::System::Com::*,
    Win32::UI::Shell::*, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    // 注册设备插入事件
    let guid_devinterface_usb_device = GUID::from_values(
        0xA5DCBF10,
        0x6530,
        0x11D2,
        [0x90, 0x1F, 0x00, 0xC0, 0x4F, 0xB9, 0x51, 0xED],
    );
    let recipient = HWND_MESSAGE.into_param();
    let mut device_notify = None;
    unsafe {
        device_notify = Some(
            RegisterDeviceNotificationW(
                recipient,
                &mut DEV_BROADCAST_DEVICEINTERFACE_W {
                    dbcc_size: std::mem::size_of::<DEV_BROADCAST_DEVICEINTERFACE_W>() as u32,
                    dbcc_devicetype: DBT_DEVTYP_DEVICEINTERFACE,
                    dbcc_reserved: 0,
                    dbcc_classguid: guid_devinterface_usb_device,
                    dbcc_name: [0; 1],
                } as *mut _ as *mut _,
                DEVICE_NOTIFY_WINDOW_HANDLE,
            )
            .assume_non_null(),
        );
    }

    // 消息循环
    loop {
        let mut msg = MSG::default();
        if unsafe { GetMessageW(&mut msg, ::core::ptr::null_mut(), 0, 0) } > 0 {
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        } else {
            break;
        }
    }

    // 注销设备插入事件
    if let Some(device_notify) = device_notify {
        unsafe {
            UnregisterDeviceNotification(device_notify);
        }
    }

    Ok(())
}

#[implement(windows::Win32::Devices::DeviceInterface::_DEVICE_INTERFACE_CHANGE_NOTIFICATION)]
struct DeviceInterfaceChangeNotification {}

#[implement(Windows::Win32::Devices::DeviceInterface::_DEVICE_INTERFACE_NOTIFY_CALLBACK)]
unsafe impl windows::Interface for DeviceInterfaceNotifyCallback {
    type Vtable = Win32::Devices::DeviceInterface::DEVICE_INTERFACE_NOTIFY_CALLBACK_V1;

    fn iid() -> windows::Guid {
        Guid::from_values(
            0x34e1fa68,
            0x4e9a,
            0x4e41,
            [0x89, 0x5b, 0x68, 0x5c, 0x7f, 0x1d, 0x0b, 0x9f],
        )
    }
}

#[windows::com_interface(
    "34e1fa68-4e9a-4e41-895b-685c7f1d0b9f",
    raw("DEVICE_INTERFACE_NOTIFY_CALLBACK_V1")
)]
trait DeviceInterfaceNotifyCallback {
    unsafe fn OnDeviceInterfaceChange(
        &self,
        notification_type: windows::Win32::DEVICE_INTERFACE_CHANGE_TYPE,
        device_interface_path: &Win32::WString,
    ) -> Result<()>;
}

impl DeviceInterfaceNotifyCallback for DeviceInterfaceChangeNotification {
    unsafe fn OnDeviceInterfaceChange(
        &self,
        notification_type: Win32::DEVICE_INTERFACE_CHANGE_TYPE,
        device_interface_path: &Win32::WString,
    ) -> windows::Result<()> {
        if notification_type == Win32::DEVICE_INTERFACE_CHANGE_TYPE_ARRIVAL {
            println!(
                "U盘插入，设备接口路径为：{}",
                device_interface_path.to_string_lossy()
            );
        } else if notification_type == Win32::DEVICE_INTERFACE_CHANGE_TYPE_REMOVAL {
            println!(
                "U盘拔出，设备接口路径为：{}",
                device_interface_path.to_string_lossy()
            );
        }
        Ok(())
    }
}

#[windows::implement(
    Windows::Win32::Devices::DeviceInterface::_DEVICE_INTERFACE_NOTIFY_SERVICE_CALLBACK
)]
struct DeviceInterfaceNotifyServiceCallback {}

#[windows::com_interface(
    "3c5e7f68-9a27-4e84-a23e-800fd8e1bde1",
    raw("DEVICE_INTERFACE_NOTIFY_SERVICE_CALLBACK_V1")
)]
trait DeviceInterfaceNotifyServiceCallback {
    unsafe fn OnDeviceInterfaceChange(
        &self,
        notification_type: Win32::DEVICE_INTERFACE_CHANGE_TYPE,
        device_interface_path: &Win32::WString,
    ) -> windows::Result<()>;
}

impl DeviceInterfaceNotifyServiceCallback for DeviceInterfaceNotifyServiceCallback {
    unsafe fn OnDeviceInterfaceChange(
        &self,
        notification_type: Win32::DEVICE_INTERFACE_CHANGE_TYPE,
        device_interface_path: &Win32::WString,
    ) -> windows::Result<()> {
        if notification_type == Win32::DEVICE_INTERFACE_CHANGE_TYPE_ARRIVAL {
            println!(
                "U盘插入，设备接口路径为：{}",
                device_interface_path.to_string_lossy()
            );
        } else if notification_type == Win32::DEVICE_INTERFACE_CHANGE_TYPE_REMOVAL {
            println!(
                "U盘拔出，设备接口路径为：{}",
                device_interface_path.to_string_lossy()
            );
        }
        Ok(())
    }
}
