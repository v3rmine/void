<template>
  <UContainer class="pt-4 sm:pt-6 lg:pt-8">
    <UFormGroup label="Language source">
      <USelect 
        color="primary"
        variant="outline"
        :options="targetLanguages"
        value="fr"
        @change="selectSourceLanguage"
        option-attribute="name"
      />
    </UFormGroup>
    <UFormGroup label="Language cible">
      <USelect 
        color="primary"
        variant="outline"
        :options="targetLanguages"
        value="en"
        @change="selectTargetLanguage"
        option-attribute="name"
      />
    </UFormGroup>
    <UAlert
      :icon="translating ? 'i-heroicons-check-circle' : 'i-heroicons-exclamation-triangle'"
      :color="translating ? 'primary' : 'amber'"
      variant="solid"
      :title="translating ? 'Listening for clipboard changes...' : 'Clipboard listener disabled'"
      :description="translating ? 'The app will now listen for changes to the clipboard.' : 'The app will not listen for changes to the clipboard.'"
      :actions="[{ 
        variant: 'outline',
        color: 'black',
        label: translating ? 'Disable' : 'Enable', 
        click: toggleTranslation,
        disabled: !clipboardBackendReady,
      }, {
        variant: 'solid',
        color: 'red',
        label: 'Clear',
        click: clearTranslations,
      }]"
    />
    <UTable 
      class="w-full"
      :ui="{ td: { base: 'max-w-[0] text-wrap' } }"
      :rows="translations"
    />
  </UContainer>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow } from '@tauri-apps/api/window';

invoke('init_clipboard_reader');

const targetLanguages = [{
  name: "Français",
  value: "fr",
}, {
  name: "Anglais",
  value: "en",
}, {
  name: "Espagnol",
  value: "es",
}, {
  name: "Italien",
  value: "it",
}, {
  name: "Allemand",
  value: "de",
}]

const translations = ref<{
  source: string,
  translation: string,
}[]>([]);
const translating = ref<boolean>(false);
const clipboardBackendReady = ref<boolean>(false);

const unlistenBackend = await appWindow.listen('backend-ready', () => {
  clipboardBackendReady.value = true;
});
const unlistenClipboard = await appWindow.listen<{
  source: string,
  translation: string,
}>('clipboard-read', (event) => {
  if (translating.value) {
    translations.value = [event.payload, ...translations.value];
  }
});

function toggleTranslation() {
  translating.value = !translating.value;
  invoke('set_clipboard_reader', { enabled: translating.value });
}
function clearTranslations() {
  translations.value = [];
}
function selectTargetLanguage(lang: any) {  
  invoke('set_target_language', { targetLanguage: lang.target.value });
}
function selectSourceLanguage(lang: any) {  
  invoke('set_source_language', { sourceLanguage: lang.target.value });
}

onBeforeUnmount(() => {
  unlistenClipboard();
  unlistenBackend();
});
</script>