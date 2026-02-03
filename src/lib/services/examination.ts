import { invoke as tauriInvoke } from "@tauri-apps/api/core";

// Helper to check if we are in a Tauri environment
const isTauri = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

async function invoke<T>(cmd: string, args?: any): Promise<T> {
    if (!isTauri()) {
        console.warn(`[Mock Mode] Tauri not detected. Simulating command: ${cmd}`, args);
        await new Promise(r => setTimeout(r, 500));

        if (cmd === 'create_examination') return Math.floor(Math.random() * 10000) as T;

        if (cmd === 'get_patient_examinations') {
            return [
                { id: 101, patient_id: args.patientId, exam_date: "2023-10-27T10:00:00", complaint: "Baş ağrısı", diagnosis: "R51 - Baş ağrısı" },
                { id: 102, patient_id: args.patientId, exam_date: "2023-09-15T14:30:00", complaint: "Öksürük", diagnosis: "J06.9 - Akut üst solunum yolu enfeksiyonu" }
            ] as T;
        }

        if (cmd === 'create_prescription') return Math.floor(Math.random() * 10000) as T;

        if (cmd === 'get_examination_prescriptions') {
            return [
                { id: 501, exam_id: args.examId, medication_name: "PAROL 500 MG", dosage: "2x1", frequency: "Günde 2" }
            ] as T;
        }

        return null as T;
    }
    return tauriInvoke(cmd, args);
}

export interface Examination {
    id?: number;
    patient_id: number;
    exam_date: string;
    complaint?: string;
    findings?: string;
    diagnosis?: string;
    treatment?: string;
}

export interface CreateExaminationInput {
    patient_id: number;
    complaint?: string;
    findings?: string;
    diagnosis?: string;
    treatment?: string;
}

export interface Prescription {
    id?: number;
    exam_id: number;
    medication_name: string;
    dosage?: string;
    frequency?: string;
}

export interface CreatePrescriptionInput {
    exam_id: number;
    medication_name: string;
    dosage?: string;
    frequency?: string;
}

export const examinationService = {
    async create(input: CreateExaminationInput): Promise<number> {
        return await invoke("create_examination", { input });
    },

    async getByPatient(patientId: number): Promise<Examination[]> {
        return await invoke("get_patient_examinations", { patientId });
    },

    async createPrescription(input: CreatePrescriptionInput): Promise<number> {
        return await invoke("create_prescription", { input });
    },

    async getPrescriptions(examId: number): Promise<Prescription[]> {
        return await invoke("get_examination_prescriptions", { examId });
    }
};
