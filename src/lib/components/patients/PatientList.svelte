<script lang="ts">
  import { onMount } from "svelte";
  import { patientService, type Patient } from "$lib/services/patient";

  let patients = $state<Patient[]>([]);
  let searchQuery = $state("");
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

  async function handleSearch() {
    if (!searchQuery) {
      await loadPatients();
      return;
    }
    patients = await patientService.search(searchQuery);
  }

  onMount(loadPatients);
</script>

<div class="space-y-6">
  <div class="flex justify-between items-center">
    <h2 class="text-2xl font-bold text-slate-800">Hasta Listesi</h2>
    <div class="flex gap-2">
      <input
        type="text"
        placeholder="Ad, Soyad veya TC No ile ara..."
        bind:value={searchQuery}
        oninput={handleSearch}
        class="px-4 py-2 border border-slate-200 rounded-lg focus:ring-2 focus:ring-primary-500 outline-none w-64"
      />
    </div>
  </div>

  {#if loading}
    <div class="flex justify-center p-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
    </div>
  {:else if patients.length === 0}
    <div class="bg-white p-12 rounded-2xl border border-dashed border-slate-300 text-center">
      <p class="text-slate-500 font-medium">Henüz hasta kaydı bulunmuyor.</p>
    </div>
  {:else}
    <div class="bg-white rounded-2xl shadow-sm border border-slate-100 overflow-hidden">
      <table class="w-full text-left border-collapse">
        <thead class="bg-slate-50 border-b border-slate-100">
          <tr>
            <th class="px-6 py-4 text-sm font-bold text-slate-600">TC No</th>
            <th class="px-6 py-4 text-sm font-bold text-slate-600">Ad Soyad</th>
            <th class="px-6 py-4 text-sm font-bold text-slate-600">Doğum Tarihi</th>
            <th class="px-6 py-4 text-sm font-bold text-slate-600">Cinsiyet</th>
            <th class="px-6 py-4 text-sm font-bold text-slate-600 text-right">İşlemler</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-50">
          {#each patients as patient}
            <tr class="hover:bg-slate-50/50 transition-colors">
              <td class="px-6 py-4 text-sm font-medium text-slate-600">{patient.tc_no}</td>
              <td class="px-6 py-4 text-sm font-bold text-slate-900">{patient.name} {patient.surname}</td>
              <td class="px-6 py-4 text-sm text-slate-600">{patient.birth_date}</td>
              <td class="px-6 py-4 text-sm text-slate-600">
                <span class="px-2 py-1 rounded-md text-xs font-bold {patient.gender === 'E' ? 'bg-blue-50 text-blue-600' : 'bg-pink-50 text-pink-600'}">
                  {patient.gender === 'E' ? 'Erkek' : 'Kadın'}
                </span>
              </td>
              <td class="px-6 py-4 text-right">
                <button class="text-primary-600 hover:text-primary-700 font-bold text-sm">Detay</button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
