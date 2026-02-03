<script lang="ts">
  import { patientService, type CreatePatientInput } from "$lib/services/patient";

  let { onAdded } = $props<{ onAdded: () => void }>();

  let form = $state<CreatePatientInput>({
    name: "",
    surname: "",
    tc_no: "",
    birth_date: "",
    gender: "E",
    phone: "",
    address: ""
  });

  let loading = $state(false);
  let error = $state("");

  async function handleSubmit() {
    loading = true;
    error = "";
    try {
      await patientService.create(form);
      // Reset form
      form = {
        name: "",
        surname: "",
        tc_no: "",
        birth_date: "",
        gender: "E",
        phone: "",
        address: ""
      };
      onAdded();
    } catch (e) {
      error = "Hasta kaydedilirken hata oluştu: " + e;
    } finally {
      loading = false;
    }
  }
</script>

<div class="bg-white p-8 rounded-3xl shadow-sm border border-slate-100">
  <h2 class="text-2xl font-bold text-slate-800 mb-6">Yeni Hasta Kaydı</h2>
  
  <form onsubmit={handleSubmit} class="space-y-6">
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="space-y-2">
        <label for="tc_no" class="text-sm font-bold text-slate-600">TC Kimlik No</label>
        <input
          id="tc_no"
          type="text"
          bind:value={form.tc_no}
          required
          maxlength="11"
          class="w-full px-4 py-2 border border-slate-200 rounded-lg focus:ring-2 focus:ring-primary-500 outline-none transition-all"
        />
      </div>
      <div class="space-y-2">
        <label for="birth_date" class="text-sm font-bold text-slate-600">Doğum Tarihi</label>
        <input
          id="birth_date"
          type="date"
          bind:value={form.birth_date}
          required
          class="w-full px-4 py-2 border border-slate-200 rounded-lg focus:ring-2 focus:ring-primary-500 outline-none transition-all"
        />
      </div>
      <div class="space-y-2">
        <label for="name" class="text-sm font-bold text-slate-600">Ad</label>
        <input
          id="name"
          type="text"
          bind:value={form.name}
          required
          class="w-full px-4 py-2 border border-slate-200 rounded-lg focus:ring-2 focus:ring-primary-500 outline-none transition-all"
        />
      </div>
      <div class="space-y-2">
        <label for="surname" class="text-sm font-bold text-slate-600">Soyad</label>
        <input
          id="surname"
          type="text"
          bind:value={form.surname}
          required
          class="w-full px-4 py-2 border border-slate-200 rounded-lg focus:ring-2 focus:ring-primary-500 outline-none transition-all"
        />
      </div>
      <div class="space-y-2">
        <label for="gender" class="text-sm font-bold text-slate-600">Cinsiyet</label>
        <select
          id="gender"
          bind:value={form.gender}
          class="w-full px-4 py-2 border border-slate-200 rounded-lg focus:ring-2 focus:ring-primary-500 outline-none transition-all"
        >
          <option value="E">Erkek</option>
          <option value="K">Kadın</option>
        </select>
      </div>
      <div class="space-y-2">
        <label for="phone" class="text-sm font-bold text-slate-600">Telefon</label>
        <input
          id="phone"
          type="text"
          bind:value={form.phone}
          class="w-full px-4 py-2 border border-slate-200 rounded-lg focus:ring-2 focus:ring-primary-500 outline-none transition-all"
        />
      </div>
    </div>

    <div class="space-y-2">
      <label for="address" class="text-sm font-bold text-slate-600">Adres</label>
      <textarea
        id="address"
        bind:value={form.address}
        rows="3"
        class="w-full px-4 py-2 border border-slate-200 rounded-lg focus:ring-2 focus:ring-primary-500 outline-none transition-all"
      ></textarea>
    </div>

    {#if error}
      <p class="text-red-500 text-sm font-medium bg-red-50 p-3 rounded-lg">{error}</p>
    {/if}

    <div class="flex justify-end gap-3">
      <button
        type="submit"
        disabled={loading}
        class="px-8 py-3 bg-primary-600 text-white font-bold rounded-xl shadow-lg shadow-primary-200 hover:bg-primary-700 disabled:opacity-50 transition-all"
      >
        {loading ? "Kaydediliyor..." : "Hastayı Kaydet"}
      </button>
    </div>
  </form>
</div>
