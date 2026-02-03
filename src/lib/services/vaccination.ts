import { invoke } from "@tauri-apps/api/core";

export interface ScheduledVaccine {
    vaccine_id: string;
    vaccine_name: string;
    dose_number: number;
    planned_date: string;
    status: 'PENDING' | 'COMPLETED' | 'DELAYED' | 'CANCELLED';
}

export const vaccinationService = {
    async calculateSchedule(birthDate: string): Promise<ScheduledVaccine[]> {
        return await invoke("calculate_vaccination_schedule", { birthDateStr: birthDate });
    }
};
