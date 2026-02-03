<script lang="ts">
  import { 
    Form, 
    FormGroup, 
    TextInput, 
    TextArea, 
    Button,
    Grid,
    Row,
    Column,
    InlineNotification,
    Tag
  } from "carbon-components-svelte";
  import { Save, Medication, Stethoscope, Add, Close } from "carbon-icons-svelte";
  import { examinationService, type CreateExaminationInput, type CreatePrescriptionInput } from "$lib/services/examination";
  import type { Patient } from "$lib/services/patient";

  let { patient, onSaved } = $props<{ patient: Patient, onSaved: () => void }>();

  let form = $state<CreateExaminationInput>({
    patient_id: patient.id!,
    complaint: "",
    findings: "",
    diagnosis: "",
    treatment: ""
  });

  // Simplified Prescription Management
  let prescriptions = $state<Partial<CreatePrescriptionInput>[]>([]);
  
  function addPrescription() {
    prescriptions.push({ medication_name: "", dosage: "", frequency: "" });
  }

  function removePrescription(index: number) {
    prescriptions = prescriptions.filter((_, i) => i !== index);
  }

  let loading = $state(false);
  let error = $state("");

  async function handleSubmit() {
    if (!form.diagnosis) {
      error = "Tanı girilmesi zorunludur.";
      return;
    }

    loading = true;
    error = "";
    try {
      const examId = await examinationService.create(form);
      
      // Save prescriptions
      for (const p of prescriptions) {
        if (p.medication_name) {
          await examinationService.createPrescription({
            exam_id: examId,
            medication_name: p.medication_name,
            dosage: p.dosage || "",
            frequency: p.frequency || ""
          });
        }
      }
      
      onSaved();
    } catch (e: any) {
      error = "Muayene kaydedilirken hata oluştu: " + e;
    } finally {
      loading = false;
    }
  }
</script>

<div class="bg-white dark:bg-zinc-900 rounded-xl border border-gray-200 dark:border-gray-800 shadow-xl overflow-hidden pane-animate">
  <header class="bg-[#0f62fe] px-6 py-4 text-white flex justify-between items-center">
    <div class="flex items-center gap-3">
      <Stethoscope size={24} />
      <div>
        <h2 class="text-sm font-black uppercase tracking-tight">MUAYENE GİRİŞİ</h2>
        <p class="text-[10px] opacity-80 uppercase font-bold">{patient.name} {patient.surname} • {patient.tc_no}</p>
      </div>
    </div>
    <Tag type="red" class="font-black m-0">AKTİF MUAYENE</Tag>
  </header>

  <Form onsubmit={handleSubmit} class="p-6">
    <Grid noGutter>
      <Row>
        <Column lg={8} md={8} sm={4} class="pr-3">
          <FormGroup>
            <TextArea
              labelText="Şikayet & Hikaye"
              placeholder="Hastanın geliş şikayeti ve tıbbi hikayesi..."
              bind:value={form.complaint}
              rows={4}
            />
          </FormGroup>
          <FormGroup>
            <TextArea
              labelText="Bulgular"
              placeholder="Fiziksel muayene bulguları..."
              bind:value={form.findings}
              rows={4}
            />
          </FormGroup>
        </Column>
        <Column lg={8} md={8} sm={4} class="pl-3 border-l border-gray-100 dark:border-gray-800">
          <FormGroup>
            <TextInput
              labelText="Tanı / ICD-10"
              placeholder="Örn: J00 - Akut nazofarenjit"
              required
              bind:value={form.diagnosis}
            />
          </FormGroup>
          <FormGroup>
            <TextArea
              labelText="Tedavi & Plan"
              placeholder="Hastaya önerilen tedavi ve izlem planı..."
              bind:value={form.treatment}
              rows={4}
            />
          </FormGroup>
        </Column>
      </Row>

      <Row class="mt-8 border-t border-gray-100 dark:border-gray-800 pt-6">
        <Column lg={16}>
          <div class="flex justify-between items-center mb-4">
            <h3 class="text-xs font-black text-gray-400 uppercase tracking-widest flex items-center gap-2">
              <Medication size={16} /> REÇETE BİLGİLERİ
            </h3>
            <Button size="small" kind="ghost" icon={Add} onclick={addPrescription}>İlaç Ekle</Button>
          </div>

          {#each prescriptions as p, i}
            <div class="flex gap-2 mb-2 items-end animate-in fade-in slide-in-from-left-2 duration-200">
               <div class="flex-1">
                  <TextInput labelText={i === 0 ? "İlaç Adı" : ""} placeholder="İlaç adı giriniz..." bind:value={p.medication_name} />
               </div>
               <div class="w-32">
                  <TextInput labelText={i === 0 ? "Doz" : ""} placeholder="Örn: 1x1" bind:value={p.dosage} />
               </div>
               <div class="w-48">
                  <TextInput labelText={i === 0 ? "Kullanım" : ""} placeholder="Örn: Tok Karnına" bind:value={p.frequency} />
               </div>
               <Button kind="danger--ghost" size="field" icon={Close} hasIconOnly iconDescription="Çıkar" onclick={() => removePrescription(i)} />
            </div>
          {/each}

          {#if prescriptions.length === 0}
            <div class="p-4 bg-gray-50 dark:bg-zinc-950/20 text-center rounded border border-dashed border-gray-200 dark:border-gray-800 text-[11px] text-gray-400 font-bold uppercase">
              Reçeteye eklenen ilaç yok.
            </div>
          {/if}
        </Column>
      </Row>
    </Grid>

    {#if error}
      <div class="mt-6">
        <InlineNotification kind="error" title="Hata:" subtitle={error} hideCloseButton />
      </div>
    {/if}

    <div class="mt-8 flex justify-end gap-3">
      <Button kind="secondary" onclick={() => onSaved()}>İPTAL</Button>
      <Button type="submit" icon={Save} disabled={loading}>
        {loading ? "KAYDEDİLİYOR..." : "MUAYENEYİ TAMAMLA"}
      </Button>
    </div>
  </Form>
</div>
