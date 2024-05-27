import { builderItem, label } from "@aaa-soft/plugin-interface/builder";
import type {
	BuilderItem,
	BuilderItemProp,
} from "@aaa-soft/plugin-interface/types/builder";

const commonProps: BuilderItemProp[] = [
	{
		id: "label",
		type: "text",
		label: label().en("Label").de("Label").build(),
	},
	{
		id: "required",
		type: "checkbox",
		label: label().en("Required").de("Pflichtfeld").build(),
		defaultValue: false,
	},
	{
		id: "class",
		type: "text",
		label: label().en("CSS Classes").de("CSS Klassen").build(),
	},
	{
		id: "name",
		type: "text",
		label: label().en("Name").de("Name").build(),
	},
];

export const formBuilderItems: BuilderItem[] = [
	builderItem({
		id: "text",
		label: label().en("Text").de("Text").build(),
		icon: "i-tabler-letter-case",
		props: [
			...commonProps,
			{
				id: "placeholder",
				type: "text",
				label: label().en("Placeholder").de("Platzhalter").build(),
			},
		],
		data: {
			label: "",
			placeholder: "",
			required: false,
			class: "grid p-2",
		},
		template: `<div class="{{data.class}}">
            <label htmlFor="text_{{index}}">
                {{data.label}}
            </label>
            <input type="text" name="text_{{index}}" required="{{data.required}}" placeholder="{{data.placeholder}}" />
        </div>`,
	}),
	builderItem({
		id: "checkbox",
		label: label().en("Checkbox").de("Kontrollkästchen").build(),
		icon: "i-tabler-checkbox",
		props: [...commonProps],
		data: {
			label: "",
			placeholder: "",
			required: false,
			class: "p-2",
		},
		template: `<div class="{{data.class}}">
            <label htmlFor="checkbox_{{index}}">
                {{data.label}}
            </label>
            <input type="checkbox" name="checkbox_{{index}}" required="{{data.required}}" />
        </div>`,
	}),
	builderItem({
		id: "textarea",
		label: label().en("Textarea").de("Textfeld").build(),
		icon: "i-tabler-text-plus",
		props: [...commonProps],
		data: {
			label: "",
			placeholder: "",
			required: false,
			class: "grid p-2",
		},
		template: `<div class="{{data.class}}">
            <label htmlFor="textarea_{{index}}">
                {{data.label}}
            </label>
            <textarea name="textarea_{{index}}" required="{{data.required}}" placeholder="{{data.placeholder}}"></textarea>
        </div>`,
	}),
	builderItem({
		id: "button",
		label: label().en("Button").de("Button").build(),
		icon: "i-tabler-square-arrow-down",
		props: [
			...commonProps,
			{
				id: "type",
				type: "buttongroup",
				label: label().en("Button type").de("Buttontyp").build(),
				options: [
					{
						label: label().en("Submit").de("Abschicken").build(),
						value: "submit",
					},
					{
						label: label().en("Reset").de("Zurücksetzen").build(),
						value: "reset",
					},
					{ label: label().en("Button").de("Button").build(), value: "button" },
				],
			},
			{
				id: "onclick",
				type: "code",
				language: "js",
				label: label().en("onClick Function").de("onClick Funktion").build(),
			},
		],
		data: { label: "Submit", class: "", type: "submit", onClick: "" },
		template: `<button type="{{data.type}}" onclick="{{data.onclick}}" class="{{data.class}}"> {{data.label}} </button>`,
	}),
];
