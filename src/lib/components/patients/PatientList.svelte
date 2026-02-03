<script lang="ts">
  import { onMount } from "svelte";
  import {
    DataTable,
    Toolbar,
    ToolbarContent,
    ToolbarSearch,
    Button,
    DataTableSkeleton
  } from "carbon-components-svelte";
  import { Stethoscope, View } from "carbon-icons-svelte";
  import { patientService, type Patient } from "$lib/services/patient";

  let { onExaminationRequested } = $props<{ onExaminationRequested?: (p: Patient) => void }>();

  let patients = $state<Patient[]>([]);
  let loading = $state(true);

  async function loadPatients() {
    loading = true;
    try {
      patients = await patientService.getAll();
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  async function handleSearch(event: any) {
    const query = event.target.value;
    if (!query) {
      await loadPatients();
      return;
    }
    patients = await patientService.search(query);
  }

  onMount(loadPatients);

  const headers = [
    { key: "tc_no", value: "TC Kimlik No" },
    { key: "full_name", value: "Ad Soyad" },
    { key: "birth_date", value: "Doğum Tarihi" },
    { key: "gender_label", value: "Cinsiyet" },
    { key: "actions", value: "İşlemler" },
  ];

  let rows = $derived(patients.map(p => ({
    id: p.id?.toString() || "",
    tc_no: p.tc_no,
    full_name: `${p.name} ${p.surname}`,
    birth_date: p.birth_date,
    gender_label: p.gender === 'E' ? 'Erkek' : 'Kadın',
    gender: p.gender,
    _original: p // Keep original object for callbacks
  })));
</script>

{#if loading}
  <div class="p-4">
    <DataTableSkeleton {headers} rows={5} />
  </div>
{:else}
  <div class="p-2 animate-in fade-in slide-in-from-bottom-2 duration-500">
    <DataTable
      title="Hasta İşlemleri"
      description="Muayene başlatmak için listeden bir hasta seçiniz."
      {headers}
      {rows}
      class="premium-table"
    >
      <Toolbar>
        <ToolbarContent>
          <ToolbarSearch oninput={handleSearch} placeholder="TC No veya İsim ile ara..." />
        </ToolbarContent>
      </Toolbar>
      
      <svelte:fragment slot="cell" let:row let:cell>
        {#if cell.key === "actions"}
          <div class="flex gap-2">
             <Button 
              size="small" 
              kind="ghost" 
              icon={Stethoscope} 
              onclick={() => onExaminationRequested?.(row._original)}
              class="hover:bg-blue-50 dark:hover:bg-blue-900/20 font-bold"
            >
              Muayene
            </Button>
            <Button size="small" kind="ghost" icon={View} class="font-bold">Detay</Button>
          </div>
        {:else if cell.key === "gender_label"}
          <span class="px-2 py-0.5 rounded-full text-[10px] font-black uppercase tracking-tight {row.gender === 'E' ? 'bg-blue-100/50 text-blue-700' : 'bg-pink-100/50 text-pink-700'}">
            {cell.value}
          </span>
        {:else if cell.key === "tc_no"}
          <span class="font-mono text-[11px] font-bold tracking-tighter opacity-70">{cell.value}</span>
        {:else}
          <span class="text-xs font-bold text-gray-700 dark:text-gray-300">{cell.value}</span>
        {/if}
      </svelte:fragment>

      <svelte:fragment slot="empty">
        <div class="p-16 text-center border-2 border-dashed border-gray-100 dark:border-gray-800 rounded-lg m-4">
          <Search size={32} class="mx-auto mb-4 text-gray-200" />
          <p class="text-sm font-black text-gray-400 uppercase tracking-widest">Kayıtlı hasta bulunamadı</p>
          <p class="text-xs text-gray-300 mt-2">Lütfen arama kriterlerinizi kontrol edin veya yeni kayıt ekleyin.</p>
        </div>
      </svelte:fragment>
    </DataTable>
  </div>
{/if}

<style>
  :global(.premium-table .bx--data-table-container) {
    background: transparent !important;
  }
  :global(.premium-table .bx--data-table) {
    background: white !important;
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 1px 3px rgba(0,0,0,0.05);
  }
  @media (prefers-color-scheme: dark) {
    :global(.premium-table .bx--data-table) {
      background: #18181b !important;
      border: 1px solid #27272a;
    }
  }
</style>
