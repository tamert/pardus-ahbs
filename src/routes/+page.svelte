<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet() {
    // greetMsg = await invoke("greet", { name });
    greetMsg = `Merhaba ${name}! AHBS Sistemine hoş geldiniz.`;
  }
</script>

<div class="max-w-6xl mx-auto p-8">
  <header class="flex justify-between items-center mb-12">
    <div>
      <h1 class="text-4xl font-extrabold text-slate-900 tracking-tight mb-2">
        Pardus AHBS <span class="text-primary-600">v2</span>
      </h1>
      <p class="text-slate-500 font-medium">Yerli ve Açık Kaynak Aile Hekimliği Bilgi Sistemi</p>
    </div>
    <div class="flex gap-4">
      <button class="px-6 py-2.5 bg-white border border-slate-200 text-slate-700 font-semibold rounded-xl shadow-sm hover:bg-slate-50 transition-all">
        Yardım
      </button>
      <button class="px-6 py-2.5 bg-primary-600 text-white font-semibold rounded-xl shadow-lg shadow-primary-200 hover:bg-primary-700 transition-all">
        Sisteme Giriş
      </button>
    </div>
  </header>

  <main class="grid grid-cols-1 md:grid-cols-3 gap-8">
    <!-- Stat Card 1 -->
    <div class="bg-white p-8 rounded-3xl shadow-sm border border-slate-100 hover:shadow-xl hover:-translate-y-1 transition-all">
      <div class="w-12 h-12 bg-blue-50 text-blue-600 rounded-2xl flex items-center justify-center mb-6">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
        </svg>
      </div>
      <h3 class="text-xl font-bold mb-2">Hasta Kayıt</h3>
      <p class="text-slate-500 leading-relaxed mb-6">Mevcut hasta kayıtlarını yönetin veya sisteme yeni bir hasta kaydı oluşturun.</p>
      <button class="text-primary-600 font-bold flex items-center gap-2 hover:gap-3 transition-all">
        Görüntüle <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg>
      </button>
    </div>

    <!-- Stat Card 2 -->
    <div class="bg-white p-8 rounded-3xl shadow-sm border border-slate-100 hover:shadow-xl hover:-translate-y-1 transition-all">
      <div class="w-12 h-12 bg-emerald-50 text-emerald-600 rounded-2xl flex items-center justify-center mb-6">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" />
        </svg>
      </div>
      <h3 class="text-xl font-bold mb-2">Muayene</h3>
      <p class="text-slate-500 leading-relaxed mb-6">Aktif muayeneleri başlatın, tıbbi geçmişi inceleyin ve reçete oluşturun.</p>
      <button class="text-primary-600 font-bold flex items-center gap-2 hover:gap-3 transition-all">
        Muayene Ekranı <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg>
      </button>
    </div>

    <!-- Stat Card 3 -->
    <div class="bg-white p-8 rounded-3xl shadow-sm border border-slate-100 hover:shadow-xl hover:-translate-y-1 transition-all">
      <div class="w-12 h-12 bg-purple-50 text-purple-600 rounded-2xl flex items-center justify-center mb-6">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
      </div>
      <h3 class="text-xl font-bold mb-2">Raporlar</h3>
      <p class="text-slate-500 leading-relaxed mb-6">Günlük, haftalık ve aylık istatistiksel raporlara ve bildirimlere erişin.</p>
      <button class="text-primary-600 font-bold flex items-center gap-2 hover:gap-3 transition-all">
        Analiz Et <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg>
      </button>
    </div>
  </main>

  <section class="mt-12 bg-white rounded-3xl border border-slate-100 p-8 shadow-sm">
    <h2 class="text-2xl font-bold mb-6">Test Alanı</h2>
    <div class="flex gap-4">
      <input 
        id="greet-input" 
        placeholder="Adınızı girin..." 
        bind:value={name}
        class="flex-1 px-4 py-3 rounded-xl border border-slate-200 outline-none focus:border-primary-500 transition-all font-medium"
      />
      <button 
        onclick={greet}
        class="px-8 py-3 bg-slate-900 text-white font-bold rounded-xl hover:bg-slate-800 transition-all"
      >
        Selamla
      </button>
    </div>
    {#if greetMsg}
      <p class="mt-6 p-4 bg-primary-50 text-primary-700 rounded-xl font-semibold animate-in fade-in slide-in-from-top-2 duration-300">
        {greetMsg}
      </p>
    {/if}
  </section>
</div>
