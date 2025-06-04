// sample front-end code for the updater
import { check } from "@tauri-apps/plugin-updater";
import { ask, message } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

export async function checkForAppUpdates(onUserClick: boolean) {
  try {
    console.log("Chequeando actualizaciones");
    const update = await check();
    console.log(update);
    if (update === null) {
      console.log("No hay actualización o error en la verificación");
      await message("Failed to check for updates.\nPlease try again later.", {
        title: "Error",
        kind: "error",
        okLabel: "OK",
      });
      return;
    }

    if (update.available) {
      console.log("Hay actualización");
      const yes = await ask(
        `Update to ${update.version} is available!\n\nRelease notes: ${update.body}`,
        {
          title: "Update Available",
          kind: "info",
          okLabel: "Update",
          cancelLabel: "Cancel",
        }
      );
      if (yes) {
        console.log("Iniciando actualización");
        await update.downloadAndInstall();
        console.log("Actualización completada");
        await message("Update installed. The app will now restart.", {
          title: "Success",
          kind: "info",
          okLabel: "OK",
        });
        await invoke("graceful_restart");
      }
    } else if (onUserClick) {
      await message("You are on the latest version. Stay awesome!", {
        title: "No Update Available",
        kind: "info",
        okLabel: "OK",
      });
    }
  } catch (error) {
    console.error("An error occurred while checking for updates:", error);
    await message(
      "An error occurred while checking for updates. Please try again later.",
      {
        title: "Error",
        kind: "error",
        okLabel: "OK",
      }
    );
  }
}
