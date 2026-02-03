<script lang="ts">
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

  function getStatusStyle(status: string) {
    switch (status) {
      case 'COMPLETED': return 'bg-emerald-100 text-emerald-700';
      case 'DELAYED': return 'bg-amber-100 text-amber-700';
      case 'CANCELLED': return 'bg-red-100 text-red-700';
      default: return 'bg-blue-50 text-blue-600';
    }
  }
</script>

<div class="space-y-8">
  <div class="bg-white p-8 rounded-3xl shadow-sm border border-slate-100">
    <h2 class="text-2xl font-bold text-slate-800 mb-6">Aşı Takvimi Hesaplayıcı</h2>
    <div class="flex gap-4 items-end">
      <div class="space-y-2 flex-1">
        <label for="birth_date_calc" class="text-sm font-bold text-slate-600">Doğum Tarihi</label>
        <input
          id="birth_date_calc"
          type="date"
          bind:value={birthDate}
          class="w-full px-4 py-2 border border-slate-200 rounded-lg focus:ring-2 focus:ring-primary-500 outline-none transition-all"
        />
      </div>
      <button
        onclick={handleCalculate}
        disabled={!birthDate || loading}
        class="px-8 py-2 bg-primary-600 text-white font-bold rounded-xl hover:bg-primary-700 disabled:opacity-50 transition-all h-[42px]"
      >
        Takvimi Oluştur
      </button>
    </div>
  </div>

  {#if loading}
    <div class="flex justify-center p-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
    </div>
  {:else if schedule.length > 0}
    <div class="bg-white rounded-2xl shadow-sm border border-slate-100 overflow-hidden animate-in fade-in slide-in-from-bottom-4 duration-500">
      <table class="w-full text-left border-collapse">
        <thead class="bg-slate-50 border-b border-slate-100">
          <tr>
            <th class="px-6 py-4 text-sm font-bold text-slate-600">Planlanan Tarih</th>
            <th class="px-6 py-4 text-sm font-bold text-slate-600">Aşı Adı</th>
            <th class="px-6 py-4 text-sm font-bold text-slate-600">Doz</th>
            <th class="px-6 py-4 text-sm font-bold text-slate-600">Durum</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-50">
          {#each schedule as item}
            <tr class="hover:bg-slate-50/50 transition-colors">
              <td class="px-6 py-4 text-sm font-bold text-slate-900">{item.planned_date}</td>
              <td class="px-6 py-4 text-sm font-medium text-slate-700">{item.vaccine_name}</td>
              <td class="px-6 py-4 text-sm text-slate-600">{item.dose_number}. Doz</td>
              <td class="px-6 py-4 text-sm">
                <span class="px-3 py-1 rounded-full text-xs font-bold {getStatusStyle(item.status)}">
                  {item.status === 'PENDING' ? 'Bekliyor' : item.status}
                </span>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
