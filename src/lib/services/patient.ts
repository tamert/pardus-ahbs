import { invoke } from "@tauri-apps/api/core";

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
