import { json } from "stream/consumers";

const utilsService = {
  formatearNombreProyecto(nombre: string): string {
    return nombre.replace(/ /g, "-").toLowerCase();
  },

  isJSON(str: string): boolean {
    try {
      const trimmed = str.trim();

      if (!trimmed || !/^[{\[]/.test(trimmed)) {
        return false;
      }

      const parsed = JSON.parse(trimmed);

      return typeof parsed === "object" && parsed !== null;
    } catch {
      return false;
    }
  },
};

export default utilsService;
