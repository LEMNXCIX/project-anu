const UtilsService = {
  formatearNombreProyecto(nombre: string): string {
    return nombre.replace(/ /g, "-").toLowerCase();
  },

  
  isJSON(str: string): boolean {
    try {
      JSON.parse(str);
      return true;
    } catch (e) {
      return false;
    }
  },
};

export default UtilsService;
