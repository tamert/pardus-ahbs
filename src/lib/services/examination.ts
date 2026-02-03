import { invoke } from "@tauri-apps/api/core";

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
