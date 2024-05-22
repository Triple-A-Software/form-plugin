import { definePlugin } from "@aaa-soft/plugin-interface";
import dayjs from "dayjs";
import { formOverviewPage } from "./settings/overview";
import { formSubmissionsPage } from "./settings/submissions";
import { createFormPage } from "./settings/form";
import { buildForm } from "./helper/utils";
export default definePlugin(async (ctx) => {
	ctx.registerHandler(["POST", "GET"], "/form", async (req) => {
		if (req.method === "POST") {
			if (req.query.id) {
				await ctx.storage.updateOne(req.query.id, {
					form: {
						...req.body,
					},
				});
			} else {
				await ctx.storage.createOne({
					form: {
						...req.body,
					},
				});
			}
			// await hooks.trigger("render")
			return new Response(null, {
				status: 200,
			});
		}
		if (req.method === "GET") {
			if (req.query.id) {
				const dbData = await ctx.storage.getOne(req.query.id);
				return dbData.data.form;
			}
			return {};
		}
		throw Error(`Method not allowed ${req.method}`);
	});

	const formHandler = ctx.registerHandler(
		["GET", "DELETE"],
		"/forms",
		async (req) => {
			if (req.method === "GET") {
				const dbData = await ctx.storage.getAllWithFilter("form");
				const allForms = dbData.map((item) => ({
					...item.data.form,
					id: item.id,
					pluginId: item.pluginId,
				}));
				return allForms;
			}
			if (req.method === "DELETE") {
				const res = await ctx.storage.deleteOne(req.query.id);
				if (res.ok) {
					return new Response(null, { status: 200 });
				}
				throw Error("Error on Deleting Record", req.query.id);
			}
			throw Error(`Method not allowed ${req.method}`);
		},
	);

	const submitHandler = ctx.registerHandler(
		["POST", "GET", "DELETE"],
		"/submit/:formId",
		async (req) => {
			if (req.method === "POST") {
				if (!req.params.formId) {
					return new Response("Form ID is missing in the URL", {
						status: 400,
					});
				}
				await ctx.storage.createOne({
					submission: {
						...req.body,
						// prefix with _ to ignore it in the Modalview for formtype 'list'
						_formId: req.params.formId,
					},
				});
				const form = await ctx.storage.getOne(req.params.formId);

				if (form.data.email) {
					ctx.sendMail(
						`[Neue Einsendung] ${req.params.formId} vom ${dayjs(
							new Date(),
						).format("DD.MM.YYYY")}`,
						form.data.email,
						`<div style="padding:1rem;">
                    <table style="width:100%;border: solid 1px;padding:1rem">
                        <tr style="border-bottom:solid 1px;text-align:center">
                            <th>Feld</th>
                            <th></th>
                        </tr>
						${Object.entries(req.body).map(
							([key, value]) => `
						<tr>
                            <td style="border-right:solid 1px;padding:1rem">
                                ${String(key).slice(0, -1)}
                            </td>
                            <td style="padding:1rem">
                                ${value}
                            </td>
                        </tr>
						`,
						)}
                    </table>
                </div>`,
					);
				}

				return new Response(null, {
					status: 302, // Statuscode für Redirect
					headers: {
						location: "/",
					},
				});
			}
			if (req.method === "DELETE") {
				const res = await ctx.storage.deleteOne(
					req.params.formId /*this is actually the submissionID*/,
				);
				if (res) {
					return new Response(null, { status: 200 });
				}
				throw Error("Error on Deleting Record");
			}
			if (req.method === "GET") {
				const allForms = await ctx.storage.getAllWithFilter("form");
				const submissions = await ctx.storage.getAllWithFilter("submission");
				const allSubmissions = [];
				for (const submission of submissions) {
					const matchingForm = allForms.find(
						(form) => form.id === Number(submission.data.submission._formId),
					);
					if (matchingForm) {
						allSubmissions.push({
							formId: submission._formId,
							id: submission.id,
							pluginId: submission.pluginId,
							name: matchingForm.data.form.name,
							recipient: matchingForm.data.form.email,
							date: dayjs(submission.createdAt).format("HH:mm, DD.MM.YYYY"),
							data: submission.data.submission,
						});
					}
				}

				return allSubmissions;
			}
		},
	);

	ctx.registerWidget(
		"formSubmissionsWidget",
		formSubmissionsPage(submitHandler),
	);

	ctx.registerSettingsPage("/submissions", formSubmissionsPage(submitHandler));
	ctx.registerSettingsPage("/overview", formOverviewPage(formHandler));
	ctx.registerSettingsPage("/create", createFormPage());

	//TODO: Trigger hook to rerender form
	//ctx.hooks.on("rerender", () => {});

	ctx.onRewrite(async (rewriter) => {
		rewriter.on("form#form_draft", {
			element(el) {
				const builder = el.getAttribute("builder");
				el.setInnerContent(`${buildForm(JSON.parse(builder))}`, { html: true });
			},
		});

		const forms = await ctx.storage.getAllWithFilter("form");
		if (!forms) return;
		for (const form of forms) {
			rewriter.on(`form#form_${form.id}`, {
				element(el) {
					el.setInnerContent(`${buildForm(form.data.form.builder)}`, {
						html: true,
					});
				},
			});
		}
	});

	ctx.registerElementType({
		name: "Form-Picker",
		icon: "i-tabler-forms",
		template:
			"<form id='form_{{ properties.select-form }}' action='/api/plugins/form-plugin/submit/{{ properties.select-form }}' class='{{properties.css-classes}}' method='POST'></form>",
		style: "",
		script: "",
		form: {
			label: {
				de: "Form-Picker",
				en: "Form-Picker",
			},
			properties: [
				{
					label: {
						de: "Formular Auswählen",
						en: "Select Form",
					},
					id: "select-form",
					type: "select",
					dataType: "string",
					options: "/api/plugins/form-plugin/forms",
				},
				{
					label: {
						de: "CSS Klassen",
						en: "CSS Classes",
					},
					id: "css-classes",
					type: "text",
				},
			],
		},
	});
});
