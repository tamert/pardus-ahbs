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
  import { View } from "carbon-icons-svelte";
  import { patientService, type Patient } from "$lib/services/patient";

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

  // Carbon DataTable format
  const headers = [
    { key: "tc_no", value: "TC Kimlik No" },
    { key: "full_name", value: "Ad Soyad" },
    { key: "birth_date", value: "Doğum Tarihi" },
    { key: "gender_label", value: "Cinsiyet" },
    { key: "overflow", value: "İşlemler", empty: true },
  ];

  let rows = $derived(patients.map(p => ({
    id: p.id?.toString() || "",
    tc_no: p.tc_no,
    full_name: `${p.name} ${p.surname}`,
    birth_date: p.birth_date,
    gender_label: p.gender === 'E' ? 'Erkek' : 'Kadın',
    gender: p.gender
  })));
</script>

{#if loading}
  <DataTableSkeleton {headers} rows={5} />
{:else}
  <DataTable
    title="Kayıtlı Hastalar"
    description="Sistemde kayıtlı olan tüm hastaların ve detaylarının listesi."
    {headers}
    {rows}
  >
    <Toolbar>
      <ToolbarContent>
        <ToolbarSearch oninput={handleSearch} placeholder="Hasta ara..." />
      </ToolbarContent>
    </Toolbar>
    
    <svelte:fragment slot="cell" let:row let:cell>
      {#if cell.key === "overflow"}
        <Button size="small" kind="ghost" icon={View}>Detay</Button>
      {:else if cell.key === "gender_label"}
        <span class="px-2 py-1 rounded text-xs font-bold {row.gender === 'E' ? 'bg-blue-100 text-blue-800' : 'bg-pink-100 text-pink-800'}">
          {cell.value}
        </span>
      {:else}
        {cell.value}
      {/if}
    </svelte:fragment>

    <svelte:fragment slot="empty">
      <div class="p-12 text-center text-gray-500">
        Henüz hasta kaydı bulunmuyor.
      </div>
    </svelte:fragment>
  </DataTable>
{/if}
