import { definePlugin } from "@cms-local/plugin-interface";
import dayjs from "dayjs";
import { formOverviewPage } from "./settings/overview";
import { formSubmissionsPage } from "./settings/submissions";
import { createFormPage } from "./settings/form";
import { textInput, checkboxInput, textAreaInput } from "./helper/utils";
export default definePlugin(async (ctx) => {
	const editHandler = ctx.registerHandler(
		["POST", "GET"],
		"/form",
		async (req) => {
			if (req.method === "POST") {
				if (req.query.id) {
					console.log("UPDATE", req.body);
					await ctx.storage.updateOne(req.query.id, {
						form: {
							...req.body,
						},
					});
				} else {
					console.log("CREATE", req.body);

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
		},
	);

	const formHandler = ctx.registerHandler(
		["GET", "DELETE"],
		"/forms",
		async (req) => {
			if (req.method === "GET") {
				const dbData = await ctx.storage.getAll();
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
				await ctx.storage.createOne(
					{
						submission: {
							...req.body,
						},
					},
					req.params.formId,
				);
				const form = await ctx.storage.getOne(req.params.formId);

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

				return new Response(null, {
					status: 302, // Statuscode für Redirect
					headers: {
						Location: "/",
					},
				});
			}
			if (req.method === "DELETE") {
				const res = await ctx.storage.deleteOne(
					req.params.formId, //this is actually the submissionID
					req.query.formId,
				);
				if (res) {
					return new Response(null, { status: 200 });
				}
				throw Error("Error on Deleting Record");
			}
			if (req.method === "GET") {
				const allForms = await ctx.storage.getAll();
				const allSubmissions = [];
				for (let i = 0; i < allForms.length; i++) {
					const submission = await ctx.storage.getAll(allForms[i].id);
					for (let k = 0; k < submission.length; k++) {
						allSubmissions.push({
							formId: allForms[i].id,
							id: submission[k].id,
							pluginId: submission[k].pluginId,
							name: allForms[i].data.form.name,
							recipient: allForms[i].data.form.email,
							date: dayjs(submission[k].createdAt).format("HH:mm, DD.MM.YYYY"),
							data: submission[k].data.submission,
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
	ctx.registerSettingsPage("/create", createFormPage(editHandler));

	//TODO: Trigger hook to rerender form
	//ctx.hooks.on("rerender", () => {});

	ctx.onRewrite(async (rewriter) => {
		const forms = await ctx.storage.getAll();
		if (!forms) return;
		for (const form of forms) {
			rewriter.on(`form[id='${form.id}']`, {
				element(el) {
					el.setInnerContent(
						`${form.data.form.builder
							.map((input, index) => {
								const { id, data } = input;
								switch (id) {
									case "text":
										return textInput(data, index);
									case "textarea":
										return textAreaInput(data, index);
									case "checkbox":
										return checkboxInput(data, index);
									//TODO: add new Elements for the Builder here
									default:
										return "";
								}
							})
							.join("")}<button type="submit">Submit</button>`,
						{ html: true },
					);
				},
			});
		}
	});

	ctx.registerElementType({
		name: "Form-Picker",
		icon: "i-tabler-forms",
		template:
			"<form id='{{ data.select-form }}' action='/api/plugins/form-plugin/submit/{{ data.select-form }}' class='{{data.css-classes}}' method='POST'></form>",
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
						end: "Select Form",
					},
					id: "select-form",
					type: "select",
					options: "/api/plugins/form-plugin/forms",
				},
				{
					label: {
						de: "CSS Klassen",
						end: "CSS Classes",
					},
					id: "css-classes",
					type: "text",
				},
			],
		},
	});
});
