const UtilsService = {
  formatearNombreProyecto(nombre: string): string {
    //TN_Telcos_Nuevo_Mejoras-en-cancelacion-de-clientes-con-Integracion-NW
    return nombre.replace(/ /g, "-").toLowerCase();
    
  },
};


export default UtilsService;