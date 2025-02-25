type CaptchaTypes = "recaptcha" | "hcaptcha" | "none" | "" | null

export async function handleCaptcha(body: Record<string, unknown>, type: CaptchaTypes, secret: string): Promise<{ captchaSuccess: boolean, data: Record<string, unknown> }> {
    switch (type) {
        case "recaptcha": {
            const { "g-recaptcha-response": recaptchaResponse, ...data } = body;

            if (!recaptchaResponse) {
                console.info("kein Recaptcha gefunden !");
            }
            try {
                const response = await fetch(
                    "https://www.google.com/recaptcha/api/siteverify",
                    {
                        method: "POST",
                        headers: {
                            "Content-Type": "application/x-www-form-urlencoded",
                        },
                        body: `secret=${secret}&response=${recaptchaResponse}`,
                    },
                );

                const captchaData = await response.json();

                if (!captchaData.success) {
                    console.info("Recaptcha Fehlgeschlagen");
                    return { captchaSuccess: false, data };
                }
            } catch (error) {
                console.error("Fehler bei der reCAPTCHA-Überprüfung:", error);
            }
            return { captchaSuccess: true, data };
        }
        case "hcaptcha": {
            const { "h-captcha-response": recaptchaResponse, ...data } = body;

            if (!recaptchaResponse) {
                console.info("kein h-captcha gefunden !");
            }
            try {
                const response = await fetch(
                    "https://api.hcaptcha.com/siteverify",
                    {
                        method: "POST",
                        headers: {
                            "Content-Type": "application/x-www-form-urlencoded",
                        },
                        body: `secret=${secret}&response=${recaptchaResponse}`,
                    },
                );

                const captchaData = await response.json();

                if (!captchaData.success) {
                    console.info("h-captcha Fehlgeschlagen");
                    return { captchaSuccess: false, data };
                }
            } catch (error) {
                console.error("Fehler bei der h-captcha-Überprüfung:", error);
            }
            return { captchaSuccess: true, data };
        }
        case "none":
            return { captchaSuccess: true, data: body };
        default:
            return { captchaSuccess: true, data: body };

    }
}