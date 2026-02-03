<script lang="ts">
  import PatientList from "$lib/components/patients/PatientList.svelte";
  import PatientAdd from "$lib/components/patients/PatientAdd.svelte";

  let activeTab = $state("dashboard"); // dashboard, patients, exams, reports
  let showAddForm = $state(false);

  let patientList: { loadPatients: () => void };

  function handlePatientAdded() {
    showAddForm = false;
    // To reload the list if it's visible, we'd normally use a store or event bus.
    // For now, let's keep it simple.
  }
</script>

<div class="max-w-6xl mx-auto p-8">
  <header class="flex justify-between items-center mb-12">
    <div>
      <h1 class="text-4xl font-extrabold text-slate-900 tracking-tight mb-2">
        Pardus AHBS <span class="text-primary-600 font-medium">v2</span>
      </h1>
      <p class="text-slate-500 font-medium italic">Yerli ve Açık Kaynak Aile Hekimliği Bilgi Sistemi</p>
    </div>
    
    <nav class="flex bg-white p-1.5 rounded-2xl shadow-sm border border-slate-100">
      <button 
        onclick={() => activeTab = "dashboard"}
        class="px-6 py-2 rounded-xl text-sm font-bold transition-all {activeTab === 'dashboard' ? 'bg-primary-50 text-primary-600' : 'text-slate-500 hover:text-slate-700'}"
      >
        Panel
      </button>
      <button 
        onclick={() => activeTab = "patients"}
        class="px-6 py-2 rounded-xl text-sm font-bold transition-all {activeTab === 'patients' ? 'bg-primary-50 text-primary-600' : 'text-slate-500 hover:text-slate-700'}"
      >
        Hastalar
      </button>
      <button 
        class="px-6 py-2 rounded-xl text-sm font-bold text-slate-300 cursor-not-allowed"
        title="Yakında"
      >
        Muayene
      </button>
    </nav>
  </header>

  {#if activeTab === "dashboard"}
    <main class="grid grid-cols-1 md:grid-cols-3 gap-8">
      <!-- Feature Card 1 -->
      <button 
        onclick={() => activeTab = "patients"}
        class="bg-white p-8 rounded-3xl shadow-sm border border-slate-100 hover:shadow-xl hover:-translate-y-1 transition-all text-left group"
      >
        <div class="w-12 h-12 bg-blue-50 text-blue-600 rounded-2xl flex items-center justify-center mb-6 group-hover:bg-blue-600 group-hover:text-white transition-colors">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
        </div>
        <h3 class="text-xl font-bold mb-2">Hasta Kayıt</h3>
        <p class="text-slate-500 leading-relaxed mb-6">Mevcut hasta kayıtlarını yönetin veya sisteme yeni bir hasta kaydı oluşturun.</p>
        <span class="text-primary-600 font-bold flex items-center gap-2 group-hover:gap-3 transition-all">
          Görüntüle <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg>
        </span>
      </button>

      <!-- Feature Card 2 -->
      <div class="bg-slate-50 p-8 rounded-3xl border border-dashed border-slate-200 opacity-60 relative group">
        <div class="w-12 h-12 bg-emerald-50 text-emerald-600 rounded-2xl flex items-center justify-center mb-6">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" />
          </svg>
        </div>
        <h3 class="text-xl font-bold mb-2">Muayene</h3>
        <p class="text-slate-500 leading-relaxed mb-6">Aktif muayeneleri başlatın, tıbbi geçmişi inceleyin ve reçete oluşturun.</p>
        <span class="inline-block px-3 py-1 bg-slate-200 text-slate-600 text-[10px] font-bold rounded-full uppercase tracking-widest">Gelecek Sürüm</span>
      </div>

      <!-- Feature Card 3 -->
      <div class="bg-slate-50 p-8 rounded-3xl border border-dashed border-slate-200 opacity-60 relative group">
        <div class="w-12 h-12 bg-purple-50 text-purple-600 rounded-2xl flex items-center justify-center mb-6">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
        </div>
        <h3 class="text-xl font-bold mb-2">Raporlar</h3>
        <p class="text-slate-500 leading-relaxed mb-6">Günlük, haftalık ve aylık istatistiksel raporlara ve bildirimlere erişin.</p>
        <span class="inline-block px-3 py-1 bg-slate-200 text-slate-600 text-[10px] font-bold rounded-full uppercase tracking-widest">Gelecek Sürüm</span>
      </div>
    </main>
  {:else if activeTab === "patients"}
    <div class="space-y-8 animate-in fade-in duration-500">
      <div class="flex justify-between items-center">
        <h2 class="text-3xl font-bold text-slate-900">Hasta Yönetimi</h2>
        <button 
          onclick={() => showAddForm = !showAddForm}
          class="px-6 py-2.5 bg-primary-600 text-white font-bold rounded-xl shadow-lg shadow-primary-200 hover:bg-primary-700 transition-all flex items-center gap-2"
        >
          {showAddForm ? 'Geri Dön' : 'Yeni Hasta Ekle'}
        </button>
      </div>

      {#if showAddForm}
        <PatientAdd onAdded={handlePatientAdded} />
      {:else}
        <PatientList />
      {/if}
    </div>
  {/if}
</div>
