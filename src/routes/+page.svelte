<script lang="ts">
  import { 
    Content, 
    Grid, 
    Row, 
    Column, 
    Button,
    DataTable,
    Toolbar,
    ToolbarContent,
    Tag,
    LocalStorage
  } from "carbon-components-svelte";
  import { 
    User, 
    UserFollow, 
    View,
    Add, 
    Close, 
    Checkmark, 
    Undo, 
    ArrowRight,
    Settings,
    Screen,
    Video,
    Warning
  } from "carbon-icons-svelte";
  
  import PatientList from "$lib/components/patients/PatientList.svelte";
  import PatientAdd from "$lib/components/patients/PatientAdd.svelte";
  import VaccineSchedule from "$lib/components/vaccination/VaccineSchedule.svelte";

  let activeTab = $state("hasta_kabul"); // kisi_islemleri, hasta_kabul, poliklinik, vs.
  let showAddForm = $state(false);

  // MOCK DATA for the 4-Pane View
  const bekleyenHastalar = [
    { id: "1", sira: 1, tip: "KESİN", tc: "11111111111", ad: "HAMİD", soyad: "KANAN", yas: 20, cinsiyet: "ERKEK" }
  ];

  const headers = [
    { key: "sira", value: "Sıra" },
    { key: "tip", value: "Tip" },
    { key: "tc", value: "TC No" },
    { key: "ad", value: "Ad" },
    { key: "soyad", value: "Soyad" },
    { key: "yas", value: "Yaş" },
    { key: "cinsiyet", value: "Cinsiyet" },
  ];
</script>

<!-- Üst Sekme Navigasyonu -->
<div class="fixed top-0 left-0 right-0 z-50 bg-[#f4f4f4] dark:bg-[#161616] border-b border-gray-300 dark:border-gray-700">
  <div class="flex overflow-x-auto no-scrollbar">
    {#each [
      {id: 'kisi_islemleri', label: 'Kişi İşlemleri'},
      {id: 'hasta_kabul', label: 'Hasta Kabul'},
      {id: 'poliklinik', label: 'Poliklinik Defteri'},
      {id: 'is_plani', label: 'İş Planı'},
      {id: 'randevu', label: 'Randevu Defteri'},
      {id: 'aile', label: 'Aile İşlemleri'},
      {id: 'veri', label: 'Veri Sorgulama'},
      {id: 'istatistik', label: 'İstatistik Çalışma'},
      {id: 'ayarlar', label: 'Program Ayarları'}
    ] as tab}
      <button 
        onclick={() => activeTab = tab.id}
        class="px-6 py-3 text-xs font-bold border-r border-gray-300 dark:border-gray-700 whitespace-nowrap transition-colors
        {activeTab === tab.id ? 'bg-white dark:bg-zinc-800 text-blue-600 border-t-2 border-t-blue-600' : 'text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-zinc-900'}"
      >
        {tab.label}
      </button>
    {/each}
  </div>

  <!-- Ribbon Araç Çubuğu -->
  <div class="flex items-center gap-1 p-1 bg-gray-100 dark:bg-zinc-900 overflow-x-auto no-scrollbar shadow-inner">
    <div class="flex flex-col items-center p-2 hover:bg-gray-200 dark:hover:bg-zinc-800 rounded cursor-pointer group">
      <Screen size={20} class="text-gray-600 dark:text-gray-400 group-hover:text-blue-600" />
      <span class="text-[10px] mt-1 font-medium">Dış Gösterim</span>
    </div>
    <div class="w-px h-10 bg-gray-300 dark:bg-gray-700 mx-1"></div>
    <div class="flex flex-col items-center p-2 hover:bg-gray-200 dark:hover:bg-zinc-800 rounded cursor-pointer group">
      <UserFollow size={20} class="text-blue-600" />
      <span class="text-[10px] mt-1 font-medium">Kişiyi Çağır</span>
    </div>
    <div class="flex flex-col items-center p-2 hover:bg-gray-200 dark:hover:bg-zinc-800 rounded cursor-pointer group">
      <ArrowRight size={20} class="text-orange-600" />
      <span class="text-[10px] mt-1 font-medium">Sona Taşı</span>
    </div>
    <div class="flex flex-col items-center p-2 hover:bg-gray-200 dark:hover:bg-zinc-800 rounded cursor-pointer group">
      <Close size={20} class="text-red-600" />
      <span class="text-[10px] mt-1 font-medium">Kişiyi Sil</span>
    </div>
    <div class="w-px h-10 bg-gray-300 dark:bg-gray-700 mx-1"></div>
    <div class="flex flex-col items-center p-2 hover:bg-gray-200 dark:hover:bg-zinc-800 rounded cursor-pointer group" onclick={() => showAddForm = !showAddForm}>
      <Add size={20} class="text-emerald-600" />
      <span class="text-[10px] mt-1 font-medium">Yeni Kayıt</span>
    </div>
    <div class="flex flex-col items-center p-2 hover:bg-gray-200 dark:hover:bg-zinc-800 rounded cursor-pointer group">
      <Checkmark size={20} class="text-blue-600" />
      <span class="text-[10px] mt-1 font-medium">Tamamlandı</span>
    </div>
    <div class="flex flex-col items-center p-2 hover:bg-gray-200 dark:hover:bg-zinc-800 rounded cursor-pointer group">
      <Undo size={20} class="text-gray-600" />
      <span class="text-[10px] mt-1 font-medium">Bekleyen</span>
    </div>
    
    <!-- Sağ Taraf Kabul Durumu -->
    <div class="ml-auto flex border border-gray-300 dark:border-gray-700 rounded overflow-hidden mr-2">
      <div class="bg-gray-100 dark:bg-zinc-800 px-3 py-1 text-[10px] font-bold border-r border-gray-300 dark:border-gray-700">KABUL DURUMU</div>
      <div class="bg-red-50 dark:bg-red-950/20 px-3 py-1 text-[10px] font-bold text-red-600">BEKLEYEN: 1</div>
      <div class="bg-emerald-50 dark:bg-emerald-950/20 px-3 py-1 text-[10px] font-bold text-emerald-600">TAMAMLANAN: 0</div>
    </div>
  </div>
</div>

<Content class="pt-28 !p-2">
  {#if activeTab === 'hasta_kabul'}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-2 h-[calc(100vh-130px)]">
      
      <!-- Üst Sol: Bekleyen Hastalar -->
      <div class="flex flex-col border border-gray-300 dark:border-gray-700 bg-white dark:bg-zinc-900 rounded shadow-sm">
        <div class="bg-blue-50 dark:bg-blue-950/20 p-2 border-b border-gray-300 dark:border-gray-700 flex justify-between items-center">
          <span class="text-xs font-bold text-blue-800 dark:text-blue-300">HASTA KABUL - BEKLEYENLER</span>
          <Button size="small" kind="ghost" iconDescription="Görünümü Kaydet" tooltipPosition="left" icon={Settings} />
        </div>
        <div class="flex-1 overflow-auto">
          {#if showAddForm}
            <div class="p-4 animate-in fade-in zoom-in duration-200">
              <PatientAdd onAdded={handlePatientAdded} />
            </div>
          {:else}
            <DataTable size="short" {headers} rows={bekleyenHastalar} />
          {/if}
        </div>
      </div>

      <!-- Üst Sağ: MHRS Randevular -->
      <div class="flex flex-col border border-gray-300 dark:border-gray-700 bg-white dark:bg-zinc-900 rounded shadow-sm">
        <div class="bg-gray-50 dark:bg-zinc-800 p-2 border-b border-gray-300 dark:border-gray-700">
          <span class="text-xs font-bold text-gray-700 dark:text-gray-300">MHRS RANDEVULAR - BEKLEYENLER</span>
        </div>
        <div class="flex-1 overflow-auto flex items-center justify-center italic text-gray-400 text-xs">
          Kayıtlı randevu bulunmamaktadır.
        </div>
      </div>

      <!-- Alt Sol: Tamamlanan Hastalar -->
      <div class="flex flex-col border border-gray-300 dark:border-gray-700 bg-white dark:bg-zinc-900 rounded shadow-sm">
        <div class="bg-emerald-50 dark:bg-emerald-950/20 p-2 border-b border-gray-300 dark:border-gray-700">
          <span class="text-xs font-bold text-emerald-800 dark:text-emerald-300">HASTA KABUL - TAMAMLANANLAR</span>
        </div>
        <div class="flex-1 overflow-auto">
          <DataTable size="short" headers={headers} rows={[]} />
          <div class="p-8 text-center text-gray-400 text-xs italic">Bugün henüz muayenesi tamamlanan hasta yok.</div>
        </div>
      </div>

      <!-- Alt Sağ: MHRS Tamamlananlar -->
      <div class="flex flex-col border border-gray-300 dark:border-gray-700 bg-white dark:bg-zinc-900 rounded shadow-sm">
        <div class="bg-gray-50 dark:bg-zinc-800 p-2 border-b border-gray-300 dark:border-gray-700">
          <span class="text-xs font-bold text-gray-700 dark:text-gray-300">MHRS RANDEVULAR - TAMAMLANANLAR</span>
        </div>
        <div class="flex-1 overflow-auto flex items-center justify-center italic text-gray-400 text-xs text-center p-4">
          MHRS üzerinden bugün gerçekleşen randevu kaydı yok.
        </div>
      </div>

    </div>
  {:else if activeTab === 'kisi_islemleri'}
    <div class="p-4 pt-4 animate-in fade-in duration-300 bg-white dark:bg-zinc-900 border border-gray-300 dark:border-gray-700 rounded min-h-screen">
       <PatientList />
    </div>
  {:else if activeTab === 'veri'}
    <div class="p-4 pt-4 animate-in fade-in duration-300 bg-white dark:bg-zinc-900 border border-gray-300 dark:border-gray-700 rounded min-h-screen">
       <VaccineSchedule />
    </div>
  {:else}
    <div class="p-12 text-center text-gray-500 italic">
      Bu modül henüz aktif değildir. Geliştirme süreci devam ediyor.
    </div>
  {/if}
</Content>

<style>
  :global(.bx--content) {
    margin-left: 0 !important;
  }
  .no-scrollbar::-webkit-scrollbar {
    display: none;
  }
  .no-scrollbar {
    -ms-overflow-style: none; /* IE and Edge */
    scrollbar-width: none; /* Firefox */
  }
</style>
