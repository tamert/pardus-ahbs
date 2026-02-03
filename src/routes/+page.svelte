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
  import ExaminationForm from "$lib/components/examination/ExaminationForm.svelte";
  import ExaminationList from "$lib/components/examination/ExaminationList.svelte";
  import type { Patient } from "$lib/services/patient";

  let activeTab = $state("hasta_kabul");
  let showAddForm = $state(false);
  let selectedPatient = $state<Patient | null>(null);
  let activeView = $state<"standard" | "examination">("standard");

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

  function startExamination(patient: Patient) {
    selectedPatient = patient;
    activeTab = "poliklinik";
    activeView = "examination";
  }

  function closeExamination() {
    activeView = "standard";
    activeTab = "hasta_kabul";
    selectedPatient = null;
  }
</script>

<!-- Modernized Header & Navigation -->
<nav class="fixed top-0 left-0 right-0 z-50 bg-[#0f62fe] text-white shadow-md">
  <div class="flex items-center justify-between px-6 h-12">
    <div class="flex items-center gap-6 h-full">
      <div class="flex items-center gap-2">
        <div class="w-2 h-5 bg-white rounded-full"></div>
        <h1 class="text-base font-black tracking-tight uppercase leading-none mt-0.5">Özgür <span class="font-normal opacity-80">AHBS</span></h1>
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

  <!-- Refined Ribbon (Conditional) -->
  {#if activeTab === 'hasta_kabul'}
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
  {/if}
</nav>

<Content class="{activeTab === 'hasta_kabul' ? 'pt-[180px]' : 'pt-[100px]'} px-8 pb-12 bg-gray-50 dark:bg-[#0c0c0c] transition-all duration-300 min-h-screen">
  {#if activeTab === 'hasta_kabul'}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 h-[calc(100vh-220px)]">
      
      <!-- Bekleyenler Pane (Key Medical Focus) -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-2xl shadow-sm overflow-hidden pane-animate transition-all duration-300 hover:shadow-md">
        <header class="flex items-center justify-between px-6 py-5 bg-white dark:bg-zinc-800/50 border-b border-gray-100 dark:border-gray-800">
          <div class="flex items-center gap-4">
            <div class="w-1.5 h-6 bg-[#0f62fe] rounded-full"></div>
            <h2 class="text-xs font-black text-gray-700 dark:text-gray-200 uppercase tracking-widest">BEKLEYEN HASTA LİSTESİ</h2>
          </div>
          <div class="flex items-center gap-2">
            <span class="flex h-2.5 w-2.5 rounded-full bg-blue-500 animate-pulse"></span>
            <span class="text-[11px] font-bold text-blue-600 bg-blue-50 dark:bg-blue-900/20 px-3 py-1 rounded-lg">1 HASTA</span>
          </div>
        </header>
        <div class="flex-1 overflow-auto bg-gray-50/30 dark:bg-zinc-950/20 p-2">
          {#if showAddForm}
            <div class="p-8 animate-in fade-in zoom-in duration-300">
              <PatientAdd onAdded={handlePatientAdded} />
            </div>
          {:else}
            <DataTable size="short" {headers} rows={bekleyenHastalar} />
          {/if}
        </div>
        <footer class="px-6 py-4 border-t border-gray-100 dark:border-gray-800 bg-white dark:bg-zinc-900 flex justify-between items-center text-[10px] font-bold text-gray-400">
           <div class="flex gap-6">
             <div class="flex flex-col">
                <span class="text-[9px] opacity-60 uppercase mb-0.5">BUGÜN</span>
                <span class="text-blue-600 font-black">01 HASTA</span>
             </div>
             <div class="flex flex-col">
                <span class="text-[9px] opacity-60 uppercase mb-0.5">GECİKEN</span>
                <span class="text-orange-500 font-black">00 HASTA</span>
             </div>
           </div>
           <Button kind="ghost" size="small" icon={Settings} hasIconOnly iconDescription="Liste Ayarları" />
        </footer>
      </section>

      <!-- Randevular Pane (Predictive View) -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-2xl shadow-sm overflow-hidden pane-animate transition-all duration-300 hover:shadow-md" style="animation-delay: 0.1s;">
        <header class="flex items-center justify-between px-6 py-5 bg-white dark:bg-zinc-800/50 border-b border-gray-100 dark:border-gray-800">
          <div class="flex items-center gap-4">
            <div class="w-1.5 h-6 bg-orange-500 rounded-full"></div>
            <h2 class="text-xs font-black text-gray-700 dark:text-gray-200 uppercase tracking-widest">MHRS RANDEVULARI</h2>
          </div>
          <Tag type="warm-gray" size="sm" class="font-bold m-0 opacity-60 italic">GÜNCEL</Tag>
        </header>
        <div class="flex-1 flex flex-col items-center justify-center p-12 bg-gray-50/50 dark:bg-zinc-950/40">
           <div class="w-20 h-20 bg-gray-100 dark:bg-zinc-800/50 rounded-3xl flex items-center justify-center mb-6 border border-gray-200 dark:border-gray-700">
              <Search size={32} class="text-gray-300" />
           </div>
           <p class="text-xs font-black text-gray-400 uppercase tracking-tight">Bekleyen randevu kaydı yok</p>
           <p class="text-[11px] text-gray-300 mt-2">Sistem otomatik senkronize ediliyor</p>
        </div>
      </section>

      <!-- Tamamlananlar Pane (History) -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-2xl shadow-sm overflow-hidden pane-animate transition-all duration-300 hover:shadow-md" style="animation-delay: 0.2s;">
        <header class="flex items-center justify-between px-6 py-5 bg-white dark:bg-zinc-800/50 border-b border-gray-100 dark:border-gray-800">
          <div class="flex items-center gap-4">
            <div class="w-1.5 h-6 bg-emerald-500 rounded-full"></div>
            <h2 class="text-xs font-black text-gray-700 dark:text-gray-200 uppercase tracking-widest">TAMAMLANAN MUAYENELER</h2>
          </div>
        </header>
        <div class="flex-1 bg-gray-50/30 dark:bg-zinc-950/20">
           <DataTable size="short" headers={headers} rows={[]} />
           <div class="p-20 text-center">
              <Checkmark size={48} class="text-gray-200 mx-auto mb-4" />
              <p class="text-xs font-black text-gray-300 uppercase italic">Henüz tamamlanan işlem yok</p>
           </div>
        </div>
      </section>

      <!-- Insights Pane (Analytics) -->
      <section class="flex flex-col bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-2xl shadow-sm overflow-hidden pane-animate transition-all duration-300 hover:shadow-md" style="animation-delay: 0.3s;">
        <header class="flex items-center justify-between px-6 py-5 bg-white dark:bg-zinc-800/50 border-b border-gray-100 dark:border-gray-800">
           <div class="flex items-center gap-4">
             <div class="w-1.5 h-6 bg-purple-500 rounded-full"></div>
             <h2 class="text-xs font-black text-gray-700 dark:text-gray-200 uppercase tracking-widest">POLİKLİNİK ÖZETİ</h2>
           </div>
        </header>
        <div class="flex-1 p-10 flex flex-col gap-8 bg-gray-50/50 dark:bg-zinc-950/10">
           <div class="flex justify-between items-center bg-white dark:bg-zinc-800 p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700">
              <span class="text-xs font-bold text-gray-500 uppercase">Bugünkü Toplam</span>
              <span class="text-4xl font-black text-blue-600">1</span>
           </div>
           <div class="flex justify-between items-center bg-white dark:bg-zinc-800 p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700">
              <span class="text-xs font-bold text-gray-500 uppercase">Görüntülü Muayene</span>
              <span class="text-4xl font-black text-emerald-500">0</span>
            </div>
         </div>
      </section>

    </div>
  {:else if activeTab === 'poliklinik'}
    {#if activeView === 'examination' && selectedPatient}
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 pb-20">
        <!-- New Exam Form -->
        <div class="lg:col-span-8">
           <ExaminationForm 
            patient={selectedPatient} 
            onSaved={closeExamination} 
           />
        </div>
        
        <!-- History Sidebar -->
        <div class="lg:col-span-4 flex flex-col gap-6">
           <div class="bg-white dark:bg-zinc-900 rounded-2xl border border-gray-200 dark:border-gray-800 shadow-sm overflow-hidden min-h-[400px]">
             <header class="bg-gray-50 dark:bg-zinc-800 px-6 py-4 border-b border-gray-100 dark:border-gray-800">
                <h3 class="text-[10px] font-black text-gray-500 uppercase tracking-widest flex items-center gap-2">
                  <Time size={16} /> GEÇMİŞ MUAYENELER
                </h3>
             </header>
             <div class="p-6">
                <ExaminationList patientId={selectedPatient.id!} />
             </div>
           </div>
        </div>
      </div>
    {:else}
      <div class="flex flex-col items-center justify-center h-[50vh] text-gray-400">
        <Stethoscope size={64} class="mb-6 opacity-50" />
        <p class="text-xl font-black tracking-tight">LÜTFEN BİR HASTA SEÇİNİZ</p>
        <p class="text-sm mt-2 opacity-70">Muayene başlatmak için hasta listesinden seçim yapın.</p>
        <Button kind="ghost" class="mt-8" onclick={() => activeTab = 'kisi_islemleri'}>HASTA LİSTESİNE GİT</Button>
      </div>
    {/if}
  {:else if activeTab === 'kisi_islemleri'}
    <div class="animate-in fade-in duration-300">
       <div class="bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-2xl shadow-lg p-8 min-h-[600px]">
          <header class="flex items-center gap-4 mb-8">
            <div class="p-3 bg-blue-50 dark:bg-blue-900/20 rounded-xl">
               <UserFollow size={24} class="text-blue-600" />
            </div>
            <div>
               <h2 class="text-lg font-black tracking-tight text-gray-800 dark:text-white">HASTA İŞLEMLERİ</h2>
               <p class="text-xs text-gray-500 font-bold uppercase tracking-wider">Tüm kayıtlı hastaların listesi ve yönetimi</p>
            </div>
          </header>
          <PatientList onExaminationRequested={startExamination} />
       </div>
    </div>
  {:else}
    <div class="flex flex-col items-center justify-center h-[50vh] text-gray-400">
      <Settings size={64} class="mb-6 opacity-50" />
      <p class="text-xl font-black tracking-tight">MODÜL HAZIRLIK AŞAMASINDA</p>
      <p class="text-sm mt-2 opacity-70">Geliştirici ekibimiz bu özellik üzerinde çalışıyor.</p>
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
