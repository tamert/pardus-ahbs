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
<nav class="fixed top-0 left-0 right-0 z-50 bg-[#0f62fe] text-white shadow-md">
  <div class="flex items-center justify-between px-6 h-12">
    <div class="flex items-center gap-6">
      <div class="flex items-center gap-2">
        <div class="w-2 h-6 bg-white rounded-full"></div>
        <h1 class="text-base font-black tracking-tight uppercase">Özgür <span class="font-normal opacity-80">AHBS</span></h1>
      </div>
      
      <div class="flex gap-1 ml-4 h-8 bg-black/10 rounded-lg p-0.5">
        {#each [
          {id: 'kisi_islemleri', label: 'KİŞİ İŞLEMLERİ'},
          {id: 'hasta_kabul', label: 'HASTA KABUL'},
          {id: 'poliklinik', label: 'POLİKLİNİK'},
          {id: 'is_plani', label: 'İŞ PLANI'}
        ] as tab}
          <button 
            onclick={() => activeTab = tab.id}
            class="px-5 text-[10px] font-bold rounded-md transition-all duration-200
            {activeTab === tab.id ? 'bg-white text-[#0f62fe] shadow-sm' : 'hover:bg-white/10 text-white/90'}"
          >
            {tab.label}
          </button>
        {/each}
      </div>
    </div>
    
    <div class="flex items-center gap-4">
      <div class="flex items-center gap-2 bg-emerald-500/20 px-3 py-1 rounded-full text-[10px] font-black text-emerald-100 border border-emerald-500/30">
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
        ÇEVRİMİÇİ
      </div>
      <Button kind="ghost" size="small" icon={Settings} hasIconOnly tooltipPosition="left" iconDescription="Ayarlar" class="text-white hover:bg-white/10" />
    </div>
  </div>

  <!-- Refined Ribbon -->
  <div class="flex items-center gap-2 px-6 py-2 bg-white dark:bg-zinc-900 border-b border-gray-200 dark:border-gray-800 h-16 shadow-sm">
    <div class="flex items-center gap-3 pr-6 border-r border-gray-100 dark:border-gray-800">
      <Button 
        kind={showAddForm ? "danger--ghost" : "primary"} 
        size="field" 
        icon={showAddForm ? Close : Add} 
        onclick={() => showAddForm = !showAddForm}
        class="font-extrabold tracking-tight px-6 rounded-lg"
      >
        {showAddForm ? 'İPTAL ET' : 'YENİ HASTA KAYDI'}
      </Button>
    </div>

    <div class="flex items-center gap-2 overflow-x-auto no-scrollbar ml-4">
      <div class="flex flex-col items-center justify-center min-w-[56px] h-11 rounded-lg hover:bg-blue-50 dark:hover:bg-blue-900/20 transition-all cursor-pointer group active:scale-95">
        <UserFollow size={18} class="text-blue-600 group-hover:-translate-y-0.5 transition-transform" />
        <span class="text-[9px] mt-1 font-bold text-gray-500 uppercase">Çağır</span>
      </div>
      <div class="flex flex-col items-center justify-center min-w-[56px] h-11 rounded-lg hover:bg-emerald-50 dark:hover:bg-emerald-900/20 transition-all cursor-pointer group active:scale-95">
        <Checkmark size={18} class="text-emerald-600 group-hover:-translate-y-0.5 transition-transform" />
        <span class="text-[9px] mt-1 font-bold text-gray-500 uppercase">Muayene</span>
      </div>
      <div class="flex flex-col items-center justify-center min-w-[56px] h-11 rounded-lg hover:bg-orange-50 dark:hover:bg-orange-900/20 transition-all cursor-pointer group active:scale-95">
        <ArrowRight size={18} class="text-orange-500 group-hover:translate-x-0.5 transition-transform" />
        <span class="text-[9px] mt-1 font-bold text-gray-500 uppercase">Ertele</span>
      </div>
      <div class="flex flex-col items-center justify-center min-w-[56px] h-11 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 transition-all cursor-pointer group active:scale-95">
        <Close size={18} class="text-red-500 group-hover:scale-110 transition-transform" />
        <span class="text-[9px] mt-1 font-bold text-gray-500 uppercase">Sil</span>
      </div>
    </div>

    <!-- Status Badge (UX Friendly) -->
    <div class="ml-auto hidden lg:flex items-center gap-6">
      <div class="flex gap-4">
        <div class="flex flex-col items-center px-4 border-r border-gray-100 dark:border-gray-800">
          <span class="text-[10px] font-black text-red-500 leading-none">01</span>
          <span class="text-[9px] font-bold text-gray-400 uppercase mt-1">Bekleyen</span>
        </div>
        <div class="flex flex-col items-center px-4">
          <span class="text-[10px] font-black text-emerald-500 leading-none">00</span>
          <span class="text-[9px] font-bold text-gray-400 uppercase mt-1">Tamamlanan</span>
        </div>
      </div>
    </div>
  </div>
</nav>

<Content class="pt-32 !p-5 bg-gray-50 dark:bg-[#0c0c0c]">
  {#if activeTab === 'hasta_kabul'}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-5 h-[calc(100vh-160px)]">
      
      <!-- Bekleyenler Pane (Key Medical Focus) -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-xl shadow-sm overflow-hidden pane-animate transition-all duration-300 hover:shadow-md">
        <header class="flex items-center justify-between px-5 py-4 bg-white dark:bg-zinc-800/50 border-b border-gray-100 dark:border-gray-800">
          <div class="flex items-center gap-3">
            <div class="w-1 h-5 bg-[#0f62fe] rounded-full"></div>
            <h2 class="text-[11px] font-black text-gray-700 dark:text-gray-200 uppercase tracking-widest">BEKLEYEN HASTA LİSTESİ</h2>
          </div>
          <div class="flex items-center gap-2">
            <span class="flex h-2 w-2 rounded-full bg-blue-500 animate-pulse"></span>
            <span class="text-[10px] font-bold text-blue-600 bg-blue-50 dark:bg-blue-900/20 px-2 py-0.5 rounded-md">1 HASTA</span>
          </div>
        </header>
        <div class="flex-1 overflow-auto bg-gray-50/30 dark:bg-zinc-950/20">
          {#if showAddForm}
            <div class="p-8 animate-in fade-in zoom-in duration-300">
              <PatientAdd onAdded={handlePatientAdded} />
            </div>
          {:else}
            <DataTable size="short" {headers} rows={bekleyenHastalar} />
          {/if}
        </div>
        <footer class="px-5 py-2.5 border-t border-gray-100 dark:border-gray-800 bg-white dark:bg-zinc-900 flex justify-between items-center text-[10px] font-bold text-gray-400">
           <div class="flex gap-4">
             <span>BUGÜN: 1</span>
             <span>GECİKEN: 0</span>
           </div>
           <Button kind="ghost" size="small" icon={Settings} hasIconOnly iconDescription="Liste Ayarları" />
        </footer>
      </section>

      <!-- Randevular Pane (Predictive View) -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-xl shadow-sm overflow-hidden pane-animate transition-all duration-300 hover:shadow-md" style="animation-delay: 0.1s;">
        <header class="flex items-center justify-between px-5 py-4 bg-white dark:bg-zinc-800/50 border-b border-gray-100 dark:border-gray-800">
          <div class="flex items-center gap-3">
            <div class="w-1 h-5 bg-orange-500 rounded-full"></div>
            <h2 class="text-[11px] font-black text-gray-700 dark:text-gray-200 uppercase tracking-widest">MHRS RANDEVULARI</h2>
          </div>
          <Tag type="warm-gray" size="sm" class="font-bold m-0 opacity-60 italic">GÜNCEL</Tag>
        </header>
        <div class="flex-1 flex flex-col items-center justify-center p-12 bg-gray-50/50 dark:bg-zinc-950/40">
           <div class="w-16 h-16 bg-gray-100 dark:bg-zinc-800/50 rounded-2xl flex items-center justify-center mb-4 border border-gray-200 dark:border-gray-700">
              <Search size={24} class="text-gray-300" />
           </div>
           <p class="text-[11px] font-black text-gray-400 uppercase tracking-tight">Bekleyen randevu kaydı yok</p>
           <p class="text-[10px] text-gray-300 mt-1">Sistem otomatik senkronize ediliyor</p>
        </div>
      </section>

      <!-- Tamamlananlar Pane (History) -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-xl shadow-sm overflow-hidden pane-animate transition-all duration-300 hover:shadow-md" style="animation-delay: 0.2s;">
        <header class="flex items-center justify-between px-5 py-4 bg-white dark:bg-zinc-800/50 border-b border-gray-100 dark:border-gray-800">
          <div class="flex items-center gap-3">
            <div class="w-1 h-5 bg-emerald-500 rounded-full"></div>
            <h2 class="text-[11px] font-black text-gray-700 dark:text-gray-200 uppercase tracking-widest">TAMAMLANAN MUAYENELER</h2>
          </div>
        </header>
        <div class="flex-1 bg-gray-50/30 dark:bg-zinc-950/20">
           <DataTable size="short" headers={headers} rows={[]} />
           <div class="p-16 text-center">
              <Checkmark size={32} class="text-gray-200 mx-auto mb-3" />
              <p class="text-[11px] font-black text-gray-300 uppercase italic">Henüz tamamlanan işlem yok</p>
           </div>
        </div>
      </section>

      <!-- Insights Pane (Analytics) -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-xl shadow-sm overflow-hidden pane-animate transition-all duration-300 hover:shadow-md" style="animation-delay: 0.3s;">
        <header class="flex items-center justify-between px-5 py-4 bg-white dark:bg-zinc-800/50 border-b border-gray-100 dark:border-gray-800">
           <div class="flex items-center gap-3">
             <div class="w-1 h-5 bg-purple-500 rounded-full"></div>
             <h2 class="text-[11px] font-black text-gray-700 dark:text-gray-200 uppercase tracking-widest">POLİKLİNİK ÖZETİ</h2>
           </div>
        </header>
        <div class="flex-1 p-8 flex flex-col gap-6 bg-gray-50/50 dark:bg-zinc-950/10">
           <div class="flex justify-between items-center bg-white dark:bg-zinc-800 p-4 rounded-lg shadow-sm border border-gray-100 dark:border-gray-700">
              <span class="text-[10px] font-bold text-gray-500 uppercase">Bugünkü Toplam</span>
              <span class="text-3xl font-black text-blue-600">1</span>
           </div>
           <div class="flex justify-between items-center bg-white dark:bg-zinc-800 p-4 rounded-lg shadow-sm border border-gray-100 dark:border-gray-700">
              <span class="text-[10px] font-bold text-gray-500 uppercase">Görüntülü Muayene</span>
              <span class="text-3xl font-black text-emerald-500">0</span>
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
