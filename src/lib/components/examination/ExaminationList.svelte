<script lang="ts">
  import { 
    DataTable, 
    DataTableSkeleton,
    Tag,
    Accordion,
    AccordionItem
  } from "carbon-components-svelte";
  import { examinationService, type Examination, type Prescription } from "$lib/services/examination";
  import { onMount } from "svelte";
  import { Medication, Time } from "carbon-icons-svelte";

  let { patientId } = $props<{ patientId: number }>();

  let examinations = $state<Examination[]>([]);
  let prescriptionsMap = $state<Record<number, Prescription[]>>({});
  let loading = $state(true);

  async function loadHistory() {
    loading = true;
    try {
       examinations = await examinationService.getByPatient(patientId);
       // Load prescriptions for each exam
       for (const exam of examinations) {
         if (exam.id) {
            prescriptionsMap[exam.id] = await examinationService.getPrescriptions(exam.id);
         }
       }
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  onMount(loadHistory);
</script>

<div class="examination-history">
  {#if loading}
    <DataTableSkeleton rows={5} />
  {:else if examinations.length === 0}
    <div class="p-12 text-center text-gray-400 font-bold uppercase italic border border-dashed border-gray-200 dark:border-gray-800 rounded-lg bg-gray-50/50">
       Geçmiş muayene kaydı bulunamadı.
    </div>
  {:else}
    <Accordion align="start">
      {#each examinations as exam}
        <AccordionItem open={false} class="border-b border-gray-100 dark:border-gray-800">
          <svelte:fragment slot="title">
            <div class="flex items-center justify-between w-full pr-4">
               <div class="flex items-center gap-3">
                  <span class="text-[11px] font-black text-blue-600 bg-blue-50 dark:bg-blue-900/20 px-2 py-1 rounded">
                    {exam.exam_date.split(' ')[0]}
                  </span>
                  <span class="text-xs font-bold text-gray-600 dark:text-gray-300">{exam.diagnosis}</span>
               </div>
               <div class="flex gap-2">
                 {#if prescriptionsMap[exam.id!]?.length > 0}
                    <Tag type="green" size="sm" class="font-bold m-0 italic">REÇETELİ</Tag>
                 {/if}
                 <Tag type="gray" size="sm" class="font-bold m-0"><Time size={12} class="mr-1" /> {exam.exam_date.split(' ')[1]}</Tag>
               </div>
            </div>
          </svelte:fragment>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6 p-4 bg-gray-50/30 dark:bg-zinc-950/20 rounded-lg">
             <div>
                <h4 class="text-[10px] font-black text-gray-400 uppercase tracking-widest mb-2">Şikayet ve Bulgular</h4>
                <p class="text-xs text-gray-600 dark:text-gray-400 whitespace-pre-wrap">{exam.complaint || '-'}</p>
                <p class="text-xs text-gray-500 mt-2 italic whitespace-pre-wrap">{exam.findings}</p>
             </div>
             <div>
                <h4 class="text-[10px] font-black text-gray-400 uppercase tracking-widest mb-2">Tanı ve Tedavi</h4>
                <p class="text-xs font-bold text-blue-600 mb-1">{exam.diagnosis}</p>
                <p class="text-xs text-gray-600 dark:text-gray-400 whitespace-pre-wrap">{exam.treatment || '-'}</p>
                
                {#if prescriptionsMap[exam.id!]?.length > 0}
                  <div class="mt-4 border-t border-gray-100 dark:border-gray-800 pt-3">
                    <h5 class="text-[10px] font-black text-emerald-600 uppercase tracking-widest mb-2 flex items-center gap-1">
                      <Medication size={14} /> REÇETE
                    </h5>
                    <ul class="space-y-1">
                      {#each prescriptionsMap[exam.id!] as p}
                        <li class="text-[11px] flex justify-between bg-white dark:bg-zinc-800 px-2 py-1 rounded shadow-sm border border-gray-100 dark:border-gray-700">
                           <span class="font-bold text-gray-700 dark:text-gray-200">{p.medication_name}</span>
                           <span class="text-gray-400">{p.dosage} • {p.frequency}</span>
                        </li>
                      {/each}
                    </ul>
                  </div>
                {/if}
             </div>
          </div>
        </AccordionItem>
      {/each}
    </Accordion>
  {/if}
</div>
