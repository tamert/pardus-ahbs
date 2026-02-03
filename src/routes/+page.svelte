<script lang="ts">
  import { 
    Content, 
    Button,
    DataTable,
    Tag,
  } from "carbon-components-svelte";
  import { 
    UserFollow, 
    Add, 
    Close, 
    Checkmark, 
    Undo, 
    ArrowRight,
    Settings,
    Screen,
    Search
  } from "carbon-icons-svelte";
  
  import PatientList from "$lib/components/patients/PatientList.svelte";
  import PatientAdd from "$lib/components/patients/PatientAdd.svelte";
  import VaccineSchedule from "$lib/components/vaccination/VaccineSchedule.svelte";

  let activeTab = $state("hasta_kabul");
  let showAddForm = $state(false);

  // MOCK DATA
  const bekleyenHastalar = [
    { id: "1", sira: 1, tip: "KESİN", tc: "11111111111", ad: "HAMİD", soyad: "KANAN", yas: 20, cinsiyet: "ERKEK" }
  ];

  const headers = [
    { key: "sira", value: "#" },
    { key: "tip", value: "TİP" },
    { key: "tc", value: "TC KİMLİK" },
    { key: "ad", value: "AD" },
    { key: "soyad", value: "SOYAD" },
    { key: "yas", value: "YAŞ" },
    { key: "cinsiyet", value: "CİNSİYET" },
  ];

  function handlePatientAdded() {
    showAddForm = false;
  }
</script>

<!-- Modernized Header & Navigation -->
<nav class="fixed top-0 left-0 right-0 z-50 bg-white dark:bg-[#161616] border-b border-gray-200 dark:border-gray-800 shadow-sm">
  <div class="flex items-center justify-between px-4 h-14 bg-[#0f62fe] text-white">
    <div class="flex items-center gap-4">
      <h1 class="text-lg font-bold tracking-tight">Pardus <span class="font-light">AHBS</span></h1>
      <div class="h-6 w-px bg-white/20 mx-2"></div>
      <div class="flex gap-1">
        {#each [
          {id: 'kisi_islemleri', label: 'KİŞİ İŞLEMLERİ'},
          {id: 'hasta_kabul', label: 'HASTA KABUL'},
          {id: 'poliklinik', label: 'POLİKLİNİK'},
          {id: 'is_plani', label: 'İŞ PLANI'}
        ] as tab}
          <button 
            onclick={() => activeTab = tab.id}
            class="px-4 py-1 text-[11px] font-bold rounded transition-all
            {activeTab === tab.id ? 'bg-white/20 shadow-inner' : 'hover:bg-white/10'}"
          >
            {tab.label}
          </button>
        {/each}
      </div>
    </div>
    
    <div class="flex items-center gap-3">
      <div class="flex items-center gap-2 bg-black/10 px-3 py-1 rounded-full text-[10px] font-bold">
        <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
        SİSTEM AKTİF
      </div>
      <Button kind="ghost" size="small" icon={Settings} hasIconOnly tooltipPosition="left" iconDescription="Ayarlar" class="text-white hover:bg-white/10" />
    </div>
  </div>

  <!-- Modernized Ribbon -->
  <div class="flex items-center gap-1 px-4 py-2 bg-[#f4f4f4] dark:bg-zinc-900 border-b border-gray-200 dark:border-gray-800 h-16">
    <div class="flex items-center gap-2 pr-4 border-r border-gray-300 dark:border-gray-700">
      <Button 
        kind="primary" 
        size="field" 
        icon={Add} 
        onclick={() => showAddForm = !showAddForm}
        class="font-bold flex-shrink-0"
      >
        {showAddForm ? 'İPTAL' : 'YENİ KAYIT'}
      </Button>
    </div>

    <div class="flex items-center gap-1 overflow-x-auto no-scrollbar ml-2">
      <div class="flex flex-col items-center justify-center min-w-[64px] h-12 rounded hover:bg-white dark:hover:bg-zinc-800 transition-all cursor-pointer group">
        <UserFollow size={20} class="text-blue-600 group-hover:scale-110 transition-transform" />
        <span class="text-[9px] mt-1 font-bold text-gray-500 uppercase tracking-tighter">Çağır</span>
      </div>
      <div class="flex flex-col items-center justify-center min-w-[64px] h-12 rounded hover:bg-white dark:hover:bg-zinc-800 transition-all cursor-pointer group">
        <Checkmark size={20} class="text-emerald-600 group-hover:scale-110 transition-transform" />
        <span class="text-[9px] mt-1 font-bold text-gray-500 uppercase tracking-tighter">Tamamla</span>
      </div>
      <div class="flex flex-col items-center justify-center min-w-[64px] h-12 rounded hover:bg-white dark:hover:bg-zinc-800 transition-all cursor-pointer group">
        <ArrowRight size={20} class="text-orange-600 group-hover:scale-110 transition-transform" />
        <span class="text-[9px] mt-1 font-bold text-gray-500 uppercase tracking-tighter">Ertele</span>
      </div>
      <div class="flex flex-col items-center justify-center min-w-[64px] h-12 rounded hover:bg-white dark:hover:bg-zinc-800 transition-all cursor-pointer group">
        <Close size={20} class="text-red-500 group-hover:scale-110 transition-transform" />
        <span class="text-[9px] mt-1 font-bold text-gray-500 uppercase tracking-tighter">Sil</span>
      </div>
      <div class="w-px h-8 bg-gray-300 dark:bg-gray-700 mx-2"></div>
      <div class="flex flex-col items-center justify-center min-w-[64px] h-12 rounded hover:bg-white dark:hover:bg-zinc-800 transition-all cursor-pointer group">
        <Screen size={20} class="text-gray-600 dark:text-gray-400 group-hover:scale-110 transition-transform" />
        <span class="text-[9px] mt-1 font-bold text-gray-500 uppercase tracking-tighter">Ekran</span>
      </div>
    </div>

    <!-- Status Badge -->
    <div class="ml-auto hidden md:flex items-center gap-4">
      <div class="flex flex-col items-end">
        <span class="text-[9px] font-bold text-gray-400 uppercase">Kabul Durumu</span>
        <div class="flex gap-2 mt-1">
          <Tag type="red" size="sm" class="font-bold m-0 h-5">BEKLEYEN: 1</Tag>
          <Tag type="green" size="sm" class="font-bold m-0 h-5">MUAYENE: 0</Tag>
        </div>
      </div>
    </div>
  </div>
</nav>

<Content class="pt-32 !p-4 bg-gray-100 dark:bg-[#121212]">
  {#if activeTab === 'hasta_kabul'}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 h-[calc(100vh-160px)]">
      
      <!-- Bekleyenler Pane -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-lg shadow-sm overflow-hidden pane-animate">
        <header class="flex items-center justify-between px-4 py-3 bg-white dark:bg-zinc-800 border-b border-gray-100 dark:border-gray-800">
          <div class="flex items-center gap-2">
            <div class="w-1.5 h-4 bg-blue-600 rounded-full"></div>
            <h2 class="text-[11px] font-bold text-gray-600 dark:text-gray-300 uppercase tracking-widest">BEKLEYEN HASTALAR</h2>
          </div>
          <Tag type="blue" size="sm" class="font-bold">AKTİF LİSTE</Tag>
        </header>
        <div class="flex-1 overflow-auto bg-gray-50 dark:bg-zinc-950/20">
          {#if showAddForm}
            <div class="p-6">
              <PatientAdd onAdded={handlePatientAdded} />
            </div>
          {:else}
            <DataTable size="short" {headers} rows={bekleyenHastalar} />
          {/if}
        </div>
        <footer class="px-4 py-2 border-t border-gray-100 dark:border-gray-800 bg-white dark:bg-zinc-900 flex justify-between">
           <span class="text-[10px] text-gray-400 font-medium italic">Toplam 1 bekleyen hasta</span>
        </footer>
      </section>

      <!-- Randevular Pane -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-lg shadow-sm overflow-hidden pane-animate" style="animation-delay: 0.1s;">
        <header class="flex items-center justify-between px-4 py-3 bg-white dark:bg-zinc-800 border-b border-gray-100 dark:border-gray-800">
          <div class="flex items-center gap-2">
            <div class="w-1.5 h-4 bg-orange-500 rounded-full"></div>
            <h2 class="text-[11px] font-bold text-gray-600 dark:text-gray-300 uppercase tracking-widest">MHRS RANDEVULARI</h2>
          </div>
          <button class="text-gray-400 hover:text-blue-600 transition-colors">
            <Search size={16} />
          </button>
        </header>
        <div class="flex-1 flex flex-col items-center justify-center p-12 bg-gray-50 dark:bg-zinc-950/50">
           <div class="w-12 h-12 bg-gray-200 dark:bg-zinc-800 rounded-full flex items-center justify-center mb-4">
              <ArrowRight size={24} class="text-gray-400" />
           </div>
           <p class="text-[11px] font-bold text-gray-400 uppercase tracking-tight">Kayıtlı randevu bulunamadı</p>
        </div>
      </section>

      <!-- Tamamlananlar Pane -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-lg shadow-sm overflow-hidden pane-animate" style="animation-delay: 0.2s;">
        <header class="flex items-center justify-between px-4 py-3 bg-white dark:bg-zinc-800 border-b border-gray-100 dark:border-gray-800">
          <div class="flex items-center gap-2">
            <div class="w-1.5 h-4 bg-emerald-500 rounded-full"></div>
            <h2 class="text-[11px] font-bold text-gray-600 dark:text-gray-300 uppercase tracking-widest">TAMAMLANANLAR</h2>
          </div>
        </header>
        <div class="flex-1 bg-gray-50 dark:bg-zinc-950/20">
           <DataTable size="short" headers={headers} rows={[]} />
           <div class="p-12 text-center">
              <p class="text-[11px] font-bold text-gray-300 uppercase italic">Muayene kaydı yok</p>
           </div>
        </div>
      </section>

      <!-- Boş Pane / İstatistik -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-lg shadow-sm overflow-hidden pane-animate" style="animation-delay: 0.3s;">
        <header class="flex items-center justify-between px-4 py-3 bg-white dark:bg-zinc-800 border-b border-gray-100 dark:border-gray-800">
           <h2 class="text-[11px] font-bold text-gray-600 dark:text-gray-300 uppercase tracking-widest">GÜNLÜK ÖZET</h2>
        </header>
        <div class="flex-1 p-6 flex flex-col gap-4">
           <div class="flex justify-between items-end border-b border-gray-100 dark:border-gray-800 pb-2">
              <span class="text-xs text-gray-400">Bugünkü Toplam Hasta</span>
              <span class="text-2xl font-bold">1</span>
           </div>
           <div class="flex justify-between items-end border-b border-gray-100 dark:border-gray-800 pb-2">
              <span class="text-xs text-gray-400">Ortalama Muayene Süresi</span>
              <span class="text-2xl font-bold">0 dk</span>
           </div>
        </div>
      </section>

    </div>
  {:else if activeTab === 'kisi_islemleri'}
    <div class="p-2 animate-in fade-in duration-300">
       <div class="bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-lg shadow-md p-4 min-h-[500px]">
          <PatientList />
       </div>
    </div>
  {:else}
    <div class="flex flex-col items-center justify-center h-[50vh] text-gray-400">
      <Settings size={48} class="mb-4 opacity-50" />
      <p class="text-lg font-bold">MODÜL HAZIRLIK AŞAMASINDA</p>
      <p class="text-sm">Geliştirici ekibimiz bu özellik üzerinde çalışıyor.</p>
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
