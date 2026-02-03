<script lang="ts">
  import { onMount } from "svelte";
  import { 
    DataTable, 
    Button, 
    Tag,
    Loading
  } from "carbon-components-svelte";
  import { Checkmark, Time, Warning } from "carbon-icons-svelte";
  import { vaccinationService, type PatientVaccination } from "$lib/services/vaccination";

  let { patientId, birthDate } = $props<{ patientId: number, birthDate: string }>();

  let schedule = $state<PatientVaccination[]>([]);
  let loading = $state(true);
  let processingId = $state<number | null>(null);

  async function loadSchedule() {
    loading = true;
    try {
      // First try to get existing
      let data = await vaccinationService.getPatientSchedule(patientId);
      
      // If empty, initialize
      if (data.length === 0 && birthDate) {
        data = await vaccinationService.initializeSchedule(patientId, birthDate);
      }
      
      schedule = data;
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  async function completeVaccine(vaccine: PatientVaccination) {
    if (!vaccine.id) return;
    processingId = vaccine.id;
    try {
      const today = new Date().toISOString().split('T')[0];
      await vaccinationService.updateStatus(vaccine.id, 'COMPLETED', today);
      
      // Update local state
      schedule = schedule.map(s => 
        s.id === vaccine.id 
          ? { ...s, status: 'COMPLETED', administered_date: today } 
          : s
      );
    } catch (e) {
      console.error("Aşı durumu güncellenemedi", e);
    } finally {
      processingId = null;
    }
  }

  onMount(() => {
    if (patientId) {
        loadSchedule();
    }
  });

  const headers = [
    { key: "scheduled_date", value: "Planlanan" },
    { key: "vaccine_name", value: "Aşı Adı" },
    { key: "status_display", value: "Durum" },
    { key: "actions", value: "İşlem", empty: true },
  ];

  let rows = $derived(schedule.map((s) => ({
    id: s.id.toString(),
    scheduled_date: s.scheduled_date,
    vaccine_name: s.vaccine_name,
    status_display: s.status,
    status: s.status,
    administered_date: s.administered_date,
    _original: s
  })));
</script>

<div class="space-y-6">
  {#if loading}
    <div class="h-64 flex items-center justify-center">
        <Loading withOverlay={false} />
    </div>
  {:else if schedule.length === 0}
    <div class="p-8 text-center border-2 border-dashed border-gray-200 dark:border-gray-700 rounded-xl">
        <Warning size={32} class="text-orange-400 mb-2 mx-auto" />
        <p class="font-bold text-gray-500">Aşı takvimi oluşturulamadı.</p>
        <p class="text-sm text-gray-400">Doğum tarihi eksik veya hatalı olabilir.</p>
    </div>
  {:else}
    <div class="bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-xl overflow-hidden shadow-sm">
        <header class="bg-blue-50 dark:bg-blue-900/10 px-6 py-4 border-b border-blue-100 dark:border-blue-800/20 flex justify-between items-center">
            <h3 class="font-bold text-blue-700 dark:text-blue-400 flex items-center gap-2">
                <Time size={20} />
                Aşı Takvimi
            </h3>
            <span class="text-xs font-bold bg-white dark:bg-zinc-800 px-3 py-1 rounded-full text-blue-600 dark:text-blue-400 border border-blue-100 dark:border-blue-900 shadow-sm">
                {schedule.filter(s => s.status === 'PENDING').length} Bekleyen
            </span>
        </header>

        <DataTable {headers} {rows} size="short">
            <svelte:fragment slot="cell" let:row let:cell>
                {#if cell.key === "status_display"}
                    {#if row.status === 'COMPLETED'}
                         <Tag type="green" class="font-bold">
                            <Checkmark size={14} class="mr-1" /> YAPILDI
                            <span class="ml-2 opacity-60 text-[10px]">{row.administered_date}</span>
                         </Tag>
                    {:else if row.status === 'MISSED'}
                         <Tag type="red" class="font-bold">GECİKTİ</Tag>
                    {:else}
                         <Tag type="blue" class="font-bold">BEKLİYOR</Tag>
                    {/if}
                {:else if cell.key === "scheduled_date"}
                    <span class="font-mono font-bold text-xs">{cell.value}</span>
                {:else if cell.key === "vaccine_name"}
                    <span class="font-bold text-xs">{cell.value}</span>
                {:else if cell.key === "actions"}
                    {#if row.status === 'PENDING'}
                        <Button 
                            kind="ghost" 
                            size="small" 
                            icon={Checkmark} 
                            disabled={processingId === parseInt(row.id)}
                            onclick={() => completeVaccine(row._original)}
                            class="font-bold text-emerald-600 hover:text-emerald-700 hover:bg-emerald-50 dark:hover:bg-emerald-900/20 rounded"
                        >
                            Uygula
                        </Button>
                    {/if}
                {:else}
                    {cell.value}
                {/if}
            </svelte:fragment>
        </DataTable>
    </div>
  {/if}
</div>
