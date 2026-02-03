<script lang="ts">
  import { 
    Form, 
    FormGroup, 
    TextInput, 
    Select, 
    SelectItem, 
    TextArea, 
    Button,
    InlineNotification,
    Grid,
    Row,
    Column
  } from "carbon-components-svelte";
  import { Save } from "carbon-icons-svelte";
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
      onAdded();
    } catch (e: any) {
      error = "Hasta kaydedilirken hata oluştu: " + e;
    } finally {
      loading = false;
    }
  }
</script>

<div class="bg-white dark:bg-zinc-900 p-8 border border-gray-200 dark:border-gray-800 shadow-lg">
  <h3 class="text-xl font-bold mb-6">Yeni Hasta Bilgileri</h3>
  
  <Form onsubmit={handleSubmit}>
    <Grid noGutter>
      <Row>
        <Column lg={8} md={8} sm={4}>
          <FormGroup>
            <TextInput
              labelText="TC Kimlik No"
              placeholder="11 haneli TC no giriniz"
              maxlength={11}
              required
              bind:value={form.tc_no}
            />
          </FormGroup>
        </Column>
        <Column lg={8} md={8} sm={4}>
          <FormGroup>
            <TextInput
              type="date"
              labelText="Doğum Tarihi"
              required
              bind:value={form.birth_date}
            />
          </FormGroup>
        </Column>
      </Row>
      <Row>
        <Column lg={8} md={8} sm={4}>
          <FormGroup>
            <TextInput
              labelText="Ad"
              placeholder="Hastanın adı"
              required
              bind:value={form.name}
            />
          </FormGroup>
        </Column>
        <Column lg={8} md={8} sm={4}>
          <FormGroup>
            <TextInput
              labelText="Soyad"
              placeholder="Hastanın soyadı"
              required
              bind:value={form.surname}
            />
          </FormGroup>
        </Column>
      </Row>
      <Row>
        <Column lg={8} md={8} sm={4}>
          <FormGroup>
            <Select labelText="Cinsiyet" bind:selected={form.gender}>
              <SelectItem value="E" text="Erkek" />
              <SelectItem value="K" text="Kadın" />
            </Select>
          </FormGroup>
        </Column>
        <Column lg={8} md={8} sm={4}>
          <FormGroup>
            <TextInput
              labelText="Telefon"
              placeholder="05xx xxx xx xx"
              bind:value={form.phone}
            />
          </FormGroup>
        </Column>
      </Row>
      <Row>
        <Column lg={16}>
          <FormGroup>
            <TextArea
              labelText="Adres"
              placeholder="Hastanın ikamet adresi..."
              bind:value={form.address}
            />
          </FormGroup>
        </Column>
      </Row>
    </Grid>

    {#if error}
      <InlineNotification
        kind="error"
        title="Hata:"
        subtitle={error}
        hideCloseButton
      />
    {/if}

    <div class="mt-8 flex justify-end">
      <Button 
        type="submit" 
        icon={Save} 
        disabled={loading}
      >
        {loading ? "Kaydediliyor..." : "Hasta Kaydını Tamamla"}
      </Button>
    </div>
  </Form>
</div>
