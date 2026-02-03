<script lang="ts">
  import { 
    Content, 
    Grid, 
    Row, 
    Column, 
    ClickableTile,
    Button,
    Tabs,
    Tab,
    TabContent
  } from "carbon-components-svelte";
  import { 
    UserMultiple, 
    Stethoscope, 
    ReportData,
    Calendar,
    Add
  } from "carbon-icons-svelte";
  
  import PatientList from "$lib/components/patients/PatientList.svelte";
  import PatientAdd from "$lib/components/patients/PatientAdd.svelte";
  import VaccineSchedule from "$lib/components/vaccination/VaccineSchedule.svelte";

  let activeIndex = $state(0);
  let showAddForm = $state(false);

  function handlePatientAdded() {
    showAddForm = false;
  }
</script>

<Content>
  <div class="mb-8">
    <h1 class="text-3xl font-bold mb-2">Pardus AHBS <span class="text-blue-600 font-medium text-xl">v2</span></h1>
    <p class="text-gray-500 italic">Yerli ve Açık Kaynak Aile Hekimliği Bilgi Sistemi (Carbon Design)</p>
  </div>

  <Tabs bind:selected={activeIndex}>
    <Tab label="Panel" />
    <Tab label="Hastalar" />
    <Tab label="Aşı Takvimi" />
    <Tab label="Muayene" disabled />
    
    <svelte:fragment slot="content">
      <TabContent>
        <Grid padding>
          <Row>
            <Column lg={4} md={4} sm={4}>
              <ClickableTile onclick={() => activeIndex = 1} class="h-full">
                <div class="p-4">
                  <UserMultiple size={32} class="mb-4 text-blue-600" />
                  <h3 class="text-xl font-bold mb-2">Hasta Kayıt</h3>
                  <p class="text-gray-500 mb-4">Mevcut hasta kayıtlarını yönetin veya yeni kayıt oluşturun.</p>
                  <Button kind="ghost" icon={Add}>Görüntüle</Button>
                </div>
              </ClickableTile>
            </Column>
            <Column lg={4} md={4} sm={4}>
              <ClickableTile onclick={() => activeIndex = 2} class="h-full">
                <div class="p-4">
                  <Calendar size={32} class="mb-4 text-emerald-600" />
                  <h3 class="text-xl font-bold mb-2">Aşı Takvimi</h3>
                  <p class="text-gray-500 mb-4">Otomatik aşı takvimi hesaplama ve takibi.</p>
                  <Button kind="ghost" icon={Add}>Görüntüle</Button>
                </div>
              </ClickableTile>
            </Column>
            <Column lg={4} md={4} sm={4}>
              <div class="bg-gray-100 p-8 rounded-lg border border-dashed border-gray-300 opacity-60 h-full">
                <Stethoscope size={32} class="mb-4 text-purple-600" />
                <h3 class="text-xl font-bold mb-2">Muayene</h3>
                <p class="text-gray-500 mb-4">Aktif muayeneleri başlatın ve reçete oluşturun.</p>
                <span class="inline-block px-3 py-1 bg-gray-200 text-gray-600 text-[10px] font-bold rounded-full uppercase">Yakında</span>
              </div>
            </Column>
          </Row>
        </Grid>
      </TabContent>
      
      <TabContent>
        <div class="p-4">
          <div class="flex justify-between items-center mb-8">
            <h2 class="text-2xl font-bold">Hasta Yönetimi</h2>
            <Button 
              kind={showAddForm ? "danger--ghost" : "primary"}
              icon={showAddForm ? undefined : Add}
              onclick={() => showAddForm = !showAddForm}
            >
              {showAddForm ? 'İptal Et' : 'Yeni Hasta Ekle'}
            </Button>
          </div>

          {#if showAddForm}
            <PatientAdd onAdded={handlePatientAdded} />
          {:else}
            <PatientList />
          {/if}
        </div>
      </TabContent>
      
      <TabContent>
        <div class="p-4">
          <VaccineSchedule />
        </div>
      </TabContent>
    </svelte:fragment>
  </Tabs>
</Content>

<style>
  :global(.bx--content) {
    background-color: transparent !important;
  }
</style>
