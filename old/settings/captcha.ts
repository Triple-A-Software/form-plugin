import type {
    RegisterHandlerReturn,
    SettingsPage,
} from "@aaa-soft/plugin-interface";
import { label } from "@aaa-soft/plugin-interface/builder";

export function captchaConfig(
    formHandler: RegisterHandlerReturn,
): SettingsPage {
    const dataHandler = formHandler.toActionHandler("GET");
    const upsertHandler = formHandler.toActionHandler("POST");

    return {
        type: "form",
        getDataHandler: {
            method: dataHandler.method,
            url: `${dataHandler.url}`,
        },
        label: label().en("Captcha Settings").de("Captcha Einstellungen").build(),
        form: {
            label: label().en("Captcha Settings").de("Captcha Einstellungen").build(),
            properties: [
                {
                    label: label().en("Secret Key").de("Secret Key").build(),
                    id: "secretKey",
                    type: "password",
                }
            ],
        },
        actions: [
            {
                primary: true,
                label: label().en("Save").de("Speichern").build(),
                handler: {
                    method: upsertHandler.method,
                    url: `${upsertHandler.url}`,
                },
            },
        ],
    };
}