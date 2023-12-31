<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTheme } from '../Settings/Settings';
import { listen } from '@tauri-apps/api/event';
import { DialogResult, DialogFieldResult, DialogAction, CheckOption, DialogField } from '@/data';
import { invoke } from '@tauri-apps/api';
import ChevronDown from "@icons/chevron-down.svg"
import PrimaryButton from '@components/PrimaryButton.vue';
import Switch from '@components/Switch.vue';
import Select from '@components/Select.vue';
import { SelectOption as SelSelectOption } from '@components/Select.vue';
import { SelectOption } from '@/data';

const updateThemeListener = ref();
const backgroundColor = ref("");
const secondaryBackgroundColor = ref("");
const tertiaryBackgroundColor = ref("");
const textColor = ref("");
const secondaryTextColor = ref("");
const accentColor = ref("");
const dialogAction = ref<DialogAction>();
const selectFieldsValues = ref<SelectFieldValue[]>([]);

onMounted(async () => {

    loadTheme();

    updateThemeListener.value = listen("update-theme", (_) => {
        loadTheme();
    });

    dialogAction.value = await invoke("get_dialog_action");

    dialogAction.value?.fields.forEach((f) => {
        selectFieldsValues.value.push({
            fieldID: f.id,
            optionID: getSelectDefaultValue(f.id)
        })
    });
});

interface SelectFieldValue {
    fieldID: string,
    optionID: string,
}


async function loadTheme() {

    let theme = await getTheme();
    backgroundColor.value = theme.background;
    secondaryBackgroundColor.value = theme.secondary_background;
    tertiaryBackgroundColor.value = theme.tertiary_background;
    textColor.value = theme.text;
    secondaryTextColor.value = theme.secondary_text;
    accentColor.value = theme.accent;
}

function getSelectDefaultValue(fieldID: string): string {

    let field = dialogAction.value?.fields.find(field => field.id === fieldID);

    return field !== undefined ? String(field.default_value) : ""
}

function getSelectOptions(fieldID: string): SelSelectOption[] {
    let field = dialogAction.value?.fields.find(field => field.id === fieldID);
    let options: SelSelectOption[] = []

    field?.options?.forEach((it) => {
        let option = it as SelectOption;

        options.push({
            value: option.id,
            text: option.value
        })
    })

    return options
}

function updateSelectValues(field: DialogField, selectOption: SelSelectOption) {
    let newSelectFieldsValues: SelectFieldValue[] = [];

    selectFieldsValues.value.forEach((sv) => {
        if (sv.fieldID === field.id) {
            newSelectFieldsValues.push({
                fieldID: sv.fieldID,
                optionID: selectOption.value
            })
        } else {
            newSelectFieldsValues.push({
                fieldID: sv.fieldID,
                optionID: sv.optionID,
            })
        }
    });

    selectFieldsValues.value = newSelectFieldsValues;

}


function finish() {

    let results: DialogFieldResult[] = [];

    dialogAction.value!!.fields!!.forEach((field) => {

        if (field.type === "Input") {
            results.push({
                field_id: field.id,
                value: (document.getElementById(field.id) as HTMLInputElement).value
            })
        }

        if (field.type === "Select") {

            results.push({
                field_id: field.id,
                value: selectFieldsValues.value.find((fv) => fv.fieldID === field.id)!!.optionID
            })
        }

        if (field.type === "Toggle") {
            results.push({
                field_id: field.id,
                value: String((document.getElementById(field.id) as HTMLInputElement).checked)
            })
        }

        if (field.type === "CheckGroup") {

            interface OptionValue {
                id: string,
                checked: boolean
            }

            var value: OptionValue[] = [];

            field.options?.forEach((option) => {
                let checked = (document.getElementById(`${field.id}-${option.id}`) as HTMLInputElement).checked
                value.push({
                    id: option.id,
                    checked: checked
                })
            });


            results.push({
                field_id: field.id,
                value: JSON.stringify(value)
            });
        }
    })

    let dialogResult: DialogResult = {
        extension_id: dialogAction.value!!.extension_id,
        action: dialogAction.value!!.action,
        results: results
    }

    invoke("write_dialog_result", { result: dialogResult })
}

</script>
<template>
    <div class="p-4 main h-screen overflow-auto flex flex-col">
        <div class="flex-grow">
            <div v-for="field in dialogAction?.fields" class="field">
                <div v-if="field.type === 'Toggle'">
                    <div class="flex">
                        <div class="flex-grow">
                            <div class="font-bold">{{ field.title }}</div>
                            <div>{{ field.description }}</div>
                        </div>
                        <div>
                            <Switch :id="field.id" :checked="Boolean(field.value)" />
                        </div>
                    </div>
                </div>
                <div v-else-if="field.type === 'CheckGroup'" :id="field.id">
                    <div class="font-bold mb-4 text-lg">{{ field.title }}</div>
                    <div v-for="option in (field.options as CheckOption[])" class="flex mb-2">
                        <div class="flex-grow">
                            <div class="font-bold">{{ option.title }}</div>
                            <div>{{ option.description }}</div>
                        </div>
                        <div>
                            <Switch :id="`${field.id}-${option.id}`" :checked="Boolean(option.checked)" />
                        </div>
                    </div>
                </div>
                <div v-else>
                    <div class="font-bold ml-2">{{ field.title }}</div>
                    <div class="ml-2">{{ field.description }}</div>

                    <input v-if="field.type === 'Input'" :value="field.value" :id="field.id" class="input"
                        :placeholder="field.placeholder">

                    <div v-if="field.type === 'Select'">
                        <Select :id="field.id"  :value="getSelectDefaultValue(field.id)" :options="getSelectOptions(field.id)"
                            @update-value="updateSelectValues(field, $event)" />
                    </div>
                </div>
            </div>
        </div>

        <PrimaryButton :text="dialogAction?.button_text ?? ''" :expand="true" @click="finish()" />
    </div>
</template>

<style scoped>
.main {
    background-color: v-bind(backgroundColor);
    color: v-bind(textColor);
}

.field {
    background-color: v-bind(secondaryBackgroundColor);
    margin-bottom: 8px;
    padding: 16px;
    border-radius: 24px;
}

.input {
    background-color: v-bind(tertiaryBackgroundColor);
    padding: 8px;
    padding-right: 16px;
    padding-left: 16px;
    width: 100%;
    border-radius: 48px;
    margin-top: 8px;
}

.input::placeholder {
    color: v-bind(secondaryTextColor);
}

.select {
    -webkit-appearance: none;
    appearance: none;
    border-radius: 48px;
    background-color: v-bind(tertiaryBackgroundColor);
    overflow: hidden;
}

.select-input {
    -webkit-appearance: none;
    appearance: none;
    background-color: v-bind(tertiaryBackgroundColor);
    width: 100%;
    outline: none;
    cursor: pointer;
    padding: 8px;
    padding-left: 16px;
    padding-right: 16px;
}

.select-chevron {
    margin-right: 16px;
    fill: v-bind(secondaryTextColor);
}
</style>