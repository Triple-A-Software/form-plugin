import { builderItem, label } from "@aaa-soft/plugin-interface/builder";
import type {
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

const formBuilderItems = [
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
						template: `<option value="{{ this.data.value }}">{{ render this.data.label }}</option>`,
					}) as BuilderItemProp
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
            <label for="{{ data.name }}">
                {{ render data.label }}
            </label>
            <select type="text" name="{{ data.name }}" {{#if data.required }}required{{/if}} placeholder="{{ render data.placeholder }}" >
			<option selected disabled value="{{render data.placeholder}}">{{ render data.placeholder }}</option>
			  	{{#each data.options}}
				<option value="{{ this.data.value }}">{{ render this.data.label }}</option>
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
			{
				id: "type",
				type: "text",
				label: label().en("Type").de("Typ").build(),
			},
		],
		data: {
			label: "",
			placeholder: "",
			required: false,
			name: "",
			type: "text",
			class: "grid p-2",
		},
		template: `<div class="{{ data.class }}">
            <label for="{{ data.name }}">
                {{ render data.label }}
            </label>
            <input type="{{ data.type }}" name="{{ data.name }}" id="{{ data.name }}" {{#if data.required }}required{{/if}} placeholder="{{ render data.placeholder }}" />
        </div>`,
	}),
	builderItem({
		id: "label",
		label: label().en("Label").de("Anzeige").build(),
		icon: "i-tabler-letter-case",
		props: [

			{
				id: "class",
				type: "text",
				label: label().en("CSS-Classes").de("CSS-Klasses").build(),
			},
			{
				id: "text",
				type: "text",
				label: label().en("Text").de("Text").build(),
			},
		],
		data: {
			text: "text",
			class: "p-2",
		},
		template: `<div class="{{ data.class }}">
		{{render data.text }}
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
            <label for="{{ data.name }}">
                {{ render data.label }}
            </label>
            <input type="checkbox" name="{{ data.name }}" id="{{ data.name }}" {{#if data.required }}required{{/if}} />
        </div>`,
	}),
	builderItem({
		id: "textarea",
		label: label().en("Textarea").de("Textfeld").build(),
		icon: "i-tabler-text-plus",
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
			name: "",
			placeholder: "",
			required: false,
			class: "grid p-2",
		},
		template: `<div class="{{ data.class }}">
            <label for="{{ data.name }}">
                {{ render data.label }}
            </label>
            <textarea name="{{ data.name }}" id="{{ data.name }}" {{#if data.required }}required{{/if}} placeholder="{{ render data.placeholder }}"></textarea>
        </div>`,
	}),
	builderItem({
		id: "button",
		label: label().en("Button").de("Button").build(),
		icon: "i-tabler-square-arrow-down",
		props: [
			...commonProps,
			{
				id: "disabled",
				type: "checkbox",
				label: label().en("Disabled").de("Deaktiviert").build(),
			},
			{
				id: "id",
				type: "text",
				label: label().en("Id").de("Id").build(),
			},
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
			name: "",
			type: "submit",
			id: "submit-btn",
			onclick: "",
			disabled: false,
		},
		template: `<button name="{{ data.name }}" id="{{data.id}}" type="{{ data.type }}" onclick="{{ data.onclick }}" class="{{ data.class }}" {{#if data.disabled}}disabled{{/if}}>
            {{ render data.label }}
        </button>`,
	}),
	builderItem({
		id: "captcha",
		label: label().en("CAPTCHA").de("CAPTCHA").build(),
		icon: "i-tabler-robot-off",
		props: [
			{
				id: "classes",
				type: "text",
				label: label().en("CSS Klassen").de("CSS Klassen").build(),

			}, {
				id: "dataSitekey",
				type: "text",
				label: label().en("data-sitekey").de("data-sitekey").build(),

			}, {
				id: "callback",
				type: "text",
				label: label().en("data-callback").de("data-callback").build(),

			}, {
				id: "expiredCallback",
				type: "text",
				label: label().en("data-expired-callback").de("data-expired-callback").build(),

			}
		],
		data: {
			dataSitekey: "",
			classes: "grid p-2",
			callback: "",
			expiredCallback: "",
		},
		template: `
		<div class="{{ data.classes }}">
          <div class="g-recaptcha" data-sitekey="{{ data.dataSitekey }}" data-callback="{{ data.callback }}"  data-expired-callback="{{ data.expiredCallback }}"></div>
		</div>
		`,
	}),
	builderItem({
		id: "hCaptcha",
		label: label().en("hCaptcha").de("hCaptcha").build(),
		icon: "i-tabler-robot-off",
		props: [
			{
				id: "classes",
				type: "text",
				label: label().en("CSS Klassen").de("CSS Klassen").build(),

			}, {
				id: "dataSitekey",
				type: "text",
				label: label().en("data-sitekey").de("data-sitekey").build(),

			}, {
				id: "callback",
				type: "text",
				label: label().en("data-callback").de("data-callback").build(),

			}, {
				id: "expiredCallback",
				type: "text",
				label: label().en("data-expired-callback").de("data-expired-callback").build(),

			}
		],
		data: {
			dataSitekey: "",
			classes: "grid p-2",
			callback: "",
			expiredCallback: "",
		},
		template: `
		<div class="{{ data.classes }}">
          <div class="h-captcha" data-sitekey="{{ data.dataSitekey }}" data-callback="{{ data.callback }}"  data-expired-callback="{{ data.expiredCallback }}"></div>
		</div>
		`,
	})
];


const builderItemsWithLayout = [...formBuilderItems, builderItem({
	id: "layout",
	label: label().en("Layout").de("Layout").build(),
	icon: "i-tabler-layout-grid",
	props: [
		...commonProps,
		{
			id: "children",
			type: "builder",
			label: label().en("Child elements").de("Kind Elemente").build(),
			items: formBuilderItems as BuilderItemProp[],
		}
	],
	data: {
		label: "",
		placeholder: "",
		required: false,
		name: "",
		class: "grid p-2",
		children: []

	},
	template: `<div class="{{ data.class }}">
 {{ render_builder(data.children) }}
        </div>`,
})]


export default builderItemsWithLayout;
