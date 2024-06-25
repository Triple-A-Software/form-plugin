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
		id: "select",
		label: label().en("Select").de("Auswahl").build(),
		icon: "i-tabler-select",
		props: [
			...commonProps,
			{
				id: "placeholder",
				type: "text",
				label: label().en("Placeholder").de("Platzhalter").build(),
			},
			{
				id: "options",
				type: "builder",
				label: label().en("Options").de("Optionen").build(),
				items: [
					builderItem({
						id: "option",
						label: label().en("Option").de("Option").build(),
						icon: "i-tabler-select",
						props: [
							{
								id: "label",
								type: "text",
								label: label().en("Label").de("Label").build(),
							}, {
								id: "value",
								type: "text",
								label: label().en("Value").de("Value").build(),
							}
						],
						data: {
							label: "",
							value: "",
						},
						template: `<option value="{{ this.data.value }}">{{ this.data.label }}</option>`,
					})
				],
			},
		],
		data: {
			label: "",
			placeholder: "",
			required: false,
			name: "",
			class: "grid p-2",
			options: []
		},
		template: `<div class="{{ data.class }}">
            <label for="{{ data.label }}">
                {{ data.label }}
            </label>
            <select type="text" name="{{ data.name }}" required="{{ data.required }}" placeholder="{{ data.placeholder }}" >
			<option selected disabled value="{{data.placeholder}}">{{ data.placeholder }}</option>
			  	{{#each data.options}}
				<option value="{{ this.data.value }}">{{ this.data.label }}</option>
  				{{/each}}
			</select>
        </div>`,


	}),
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
			name: "",
			class: "grid p-2",
		},
		template: `<div class="{{ data.class }}">
            <label for="{{ data.label }}">
                {{ data.label }}
            </label>
            <input type="text" name="{{ data.name }}" required="{{ data.required }}" placeholder="{{ data.placeholder }}" />
        </div>`,
	}),
	builderItem({
		id: "checkbox",
		label: label().en("Checkbox").de("Kontrollkästchen").build(),
		icon: "i-tabler-checkbox",
		props: [...commonProps],
		data: {
			label: "",
			name: "",
			placeholder: "",
			required: false,
			class: "p-2",
		},
		template: `<div class="{{ data.class }}">
            <label for="{{ data.label }}">
                {{ data.label }}
            </label>
            <input type="checkbox" name="{{ data.name }}" required="{{ data.required }}" />
        </div>`,
	}),
	builderItem({
		id: "textarea",
		label: label().en("Textarea").de("Textfeld").build(),
		icon: "i-tabler-text-plus",
		props: [...commonProps],
		data: {
			label: "",
			name: "",
			placeholder: "",
			required: false,
			class: "grid p-2",
		},
		template: `<div class="{{ data.class }}">
            <label for="{{ data.label }}">
                {{ data.label }}
            </label>
            <textarea name="{{ data.name }}" required="{{ data.required }}" placeholder="{{ data.placeholder }}"></textarea>
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
		data: {
			label: "Submit",
			class: "",
			type: "submit",
			onClick: "",
		},
		template: `<button type="{{ data.type }}" onclick="{{ data.onclick }}" class="{{ data.class }}">
            {{ data.label }}
        </button>`,
	}),
];
