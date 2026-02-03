import { invoke as tauriInvoke } from "@tauri-apps/api/core";

// Helper to check if we are in a Tauri environment
const isTauri = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

async function invoke<T>(cmd: string, args?: any): Promise<T> {
    if (!isTauri()) {
        console.warn(`[Mock Mode] Tauri not detected. Simulating command: ${cmd}`, args);

        // Mock Responses
        await new Promise(r => setTimeout(r, 500)); // Simulate network delay

        if (cmd === 'create_patient') return Math.floor(Math.random() * 10000) as T;

        if (cmd === 'get_patients') {
            return [
                { id: 1, tc_no: "11111111111", name: "HAMİD", surname: "KANAN", birth_date: "2004-01-01", gender: "E" },
                { id: 2, tc_no: "22222222222", name: "AYŞE", surname: "YILMAZ", birth_date: "1995-05-15", gender: "K" }
            ] as T;
        }

        if (cmd === 'search_patient') {
            return [
                { id: 1, tc_no: "11111111111", name: "HAMİD", surname: "KANAN", birth_date: "2004-01-01", gender: "E" }
            ] as T;
        }

        return null as T;
    }
    return tauriInvoke(cmd, args);
}

export interface Patient {
    id?: number;
    name: string;
    surname: string;
    tc_no: string;
    birth_date: string;
    gender: string;
    phone?: string;
    address?: string;
}

export interface CreatePatientInput {
    name: string;
    surname: string;
    tc_no: string;
    birth_date: string;
    gender: string;
    phone?: string;
    address?: string;
}

export const patientService = {
    async create(input: CreatePatientInput): Promise<number> {
        return await invoke("create_patient", { input });
    },

    async getAll(): Promise<Patient[]> {
        return await invoke("get_patients");
    },

    async search(query: string): Promise<Patient[]> {
        return await invoke("search_patient", { query });
    }
};
