import { invoke } from "@tauri-apps/api/core";

export interface AuthenticateRequest {
    url: string;
    callbackScheme: string;
}

export interface AuthenticateResponse {
    callbackUrl: string;
}

export async function authenticate(
    request: AuthenticateRequest
): Promise<AuthenticateResponse> {
    return await invoke<AuthenticateResponse>("plugin:web-auth|authenticate", {
        payload: request,
    });
}
