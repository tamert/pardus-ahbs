import { invoke as tauriInvoke } from "@tauri-apps/api/core";

// Helper to check if we are in a Tauri environment
const isTauri = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

async function invoke<T>(cmd: string, args?: any): Promise<T> {
    if (!isTauri()) {
        console.warn(`[Mock Mode] Tauri not detected. Simulating command: ${cmd}`, args);
        await new Promise(r => setTimeout(r, 500));

        if (cmd === 'get_vaccine_definitions') {
            return [
                { id: 1, code: 'HEPB1', name: 'Hepatit B 1. Doz', month_offset: 0 },
                { id: 2, code: 'HEPB2', name: 'Hepatit B 2. Doz', month_offset: 1 },
                { id: 3, code: 'BCG', name: 'Verem (BCG)', month_offset: 2 },
                { id: 4, code: 'KKK', name: 'KKK 1. Doz', month_offset: 12 },
            ] as T;
        }

        if (cmd === 'get_patient_vaccinations' || cmd === 'initialize_patient_schedule') {
            // Mock schedule based on args or random
            return [
                { id: 1, patient_id: args?.patientId || 1, vaccine_code: 'HEPB1', vaccine_name: 'Hepatit B 1. Doz', scheduled_date: '2024-01-01', status: 'COMPLETED', administered_date: '2024-01-02' },
                { id: 2, patient_id: args?.patientId || 1, vaccine_code: 'HEPB2', vaccine_name: 'Hepatit B 2. Doz', scheduled_date: '2024-02-01', status: 'PENDING', administered_date: null },
                { id: 3, patient_id: args?.patientId || 1, vaccine_code: 'BCG', vaccine_name: 'Verem (BCG)', scheduled_date: '2024-03-01', status: 'PENDING', administered_date: null },
            ] as T;
        }

        if (cmd === 'update_vaccination_status') {
            return null as T;
        }

        return null as T;
    }
    return tauriInvoke(cmd, args);
}

export interface VaccineDefinition {
    id: number;
    code: string;
    name: string;
    month_offset: number;
    description?: string;
}

export interface PatientVaccination {
    id: number;
    patient_id: number;
    vaccine_code: string;
    vaccine_name: string;
    scheduled_date: string;
    administered_date?: string | null;
    status: 'PENDING' | 'COMPLETED' | 'MISSED' | 'DELAYED';
    lot_no?: string | null;
    injection_site?: string | null;
    notes?: string;
}

export const vaccinationService = {
    async getDefinitions(): Promise<VaccineDefinition[]> {
        return await invoke("get_vaccine_definitions");
    },

    async getPatientSchedule(patientId: number): Promise<PatientVaccination[]> {
        return await invoke("get_patient_vaccinations", { patientId });
    },

    async initializeSchedule(patientId: number, birthDateStr: string): Promise<PatientVaccination[]> {
        return await invoke("initialize_patient_schedule", { patientId, birthDateStr });
    },

    async updateStatus(
        id: number,
        status: string,
        administeredDate: string | null,
        lotNo?: string | null,
        injectionSite?: string | null
    ): Promise<void> {
        return await invoke("update_vaccination_status", {
            id,
            status,
            administeredDate,
            lotNo,
            injectionSite
        });
    }
};
