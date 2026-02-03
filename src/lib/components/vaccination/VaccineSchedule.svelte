<script lang="ts">
  import { 
    DataTable, 
    TextInput, 
    Button, 
    Grid, 
    Row, 
    Column,
    Tag
  } from "carbon-components-svelte";
  import { Calculator } from "carbon-icons-svelte";
  import { vaccinationService, type ScheduledVaccine } from "$lib/services/vaccination";

  let birthDate = $state("");
  let schedule = $state<ScheduledVaccine[]>([]);
  let loading = $state(false);

  async function handleCalculate() {
    if (!birthDate) return;
    loading = true;
    try {
      schedule = await vaccinationService.calculateSchedule(birthDate);
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  const headers = [
    { key: "planned_date", value: "Planlanan Tarih" },
    { key: "vaccine_name", value: "Aşı Adı" },
    { key: "dose_label", value: "Doz" },
    { key: "status_tag", value: "Durum", empty: true },
  ];

  let rows = $derived(schedule.map((s, i) => ({
    id: i.toString(),
    planned_date: s.planned_date,
    vaccine_name: s.vaccine_name,
    dose_label: `${s.dose_number}. Doz`,
    status: s.status
  })));

  function getStatusTagType(status: string) {
    switch (status) {
      case 'COMPLETED': return 'green';
      case 'DELAYED': return 'warm-gray';
      case 'CANCELLED': return 'red';
      default: return 'blue';
    }
  }

  function getStatusText(status: string) {
    if (status === 'PENDING') return 'Bekliyor';
    return status;
  }
</script>

<div class="space-y-8">
  <div class="bg-white dark:bg-zinc-900 p-8 border border-gray-200 dark:border-gray-800 shadow-md">
    <h3 class="text-xl font-bold mb-6">Aşı Zamanı Hesapla</h3>
    <Grid noGutter>
      <Row items="end">
        <Column lg={12} md={12} sm={4}>
          <TextInput
            type="date"
            labelText="Hastanın Doğum Tarihi"
            bind:value={birthDate}
          />
        </Column>
        <Column lg={4} md={4} sm={4}>
          <Button
            icon={Calculator}
            onclick={handleCalculate}
            disabled={!birthDate || loading}
            class="w-full"
          >
            Hesapla
          </Button>
        </Column>
      </Row>
    </Grid>
  </div>

  {#if schedule.length > 0}
    <div class="animate-in fade-in duration-500">
      <DataTable
        title="Hesaplanan Aşı Takvimi"
        description="Hastanın yaşına göre planlanan aşıların dökümü."
        {headers}
        {rows}
      >
        <svelte:fragment slot="cell" let:row let:cell>
          {#if cell.key === "status_tag"}
            <Tag type={getStatusTagType(row.status)}>
              {getStatusText(row.status)}
            </Tag>
          {:else if cell.key === "planned_date"}
            <span class="font-bold">{cell.value}</span>
          {:else}
            {cell.value}
          {/if}
        </svelte:fragment>
      </DataTable>
    </div>
  {/if}
</div>
