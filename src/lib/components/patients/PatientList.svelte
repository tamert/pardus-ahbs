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
  <DataTableSkeleton {headers} rows={5} />
{:else}
  <DataTable
    title="Hasta İşlemleri"
    description="Muayene başlatmak için listeden bir hasta seçiniz."
    {headers}
    {rows}
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
          >
            Muayene
          </Button>
          <Button size="small" kind="ghost" icon={View}>Detay</Button>
        </div>
      {:else if cell.key === "gender_label"}
        <span class="px-2 py-1 rounded text-xs font-bold {row.gender === 'E' ? 'bg-blue-100/50 text-blue-700' : 'bg-pink-100/50 text-pink-700'}">
          {cell.value}
        </span>
      {:else}
        {cell.value}
      {/if}
    </svelte:fragment>

    <svelte:fragment slot="empty">
      <div class="p-12 text-center text-gray-500 italic font-bold">
        Hasta kaydı bulunamadı.
      </div>
    </svelte:fragment>
  </DataTable>
{/if}
