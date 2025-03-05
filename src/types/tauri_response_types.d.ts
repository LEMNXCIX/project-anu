export type TauriResponse<T> = {
    success: boolean;
    error: boolean;
    message: string;
    data: T;
    error_details: any[];
}
export type DefaultResult<T> = {
    result: T;
}
export function createTauriResponse<T>(): TauriResponse {
    return {
        success: false,
        error: false,
        message: "",
        data: {} as T,
        error_details: []
    };
}
