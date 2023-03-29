import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
import { getName } from '@tauri-apps/api/app';

export async function notify(body) {
  if (!await isPermissionGranted()) {
    const permissionResult = await requestPermission();

    if (permissionResult !== 'granted') {
      return;
    }
  }

  const appName = await getName();

  sendNotification({ body: body, title: appName });
}