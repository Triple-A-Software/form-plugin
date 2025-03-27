const CR_BASE_URL = "https://rest.cleverreach.com";
const CR_TOKEN_URL = `${CR_BASE_URL}/oauth/token.php`;
const CR_API_URL = `${CR_BASE_URL}/v3`;


let accessToken: string | null | undefined;


const updateAccessToken = async (client_id: string, client_secret: string) => {
    if (!client_id || !client_secret) return;
    const response = await fetch(CR_TOKEN_URL, {
        method: "POST",
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: new URLSearchParams({
            "client_id": client_id,
            "client_secret": client_secret,
            "grant_type": "client_credentials",
        })
    });
    if (response.status !== 200) return;
    accessToken = (await response.json()).access_token;
};



export const createReceiver = async (client_id: string, client_secret: string, groupId: string, email: string, firstName: string, lastName: string, salutation: string ) => {
    
    if (!accessToken) await updateAccessToken(client_id, client_secret);
    const res = await fetch(`${CR_API_URL}/groups/${groupId}/receivers`,
        {
            method: "POST",
            headers: {
                Authorization: `Bearer ${accessToken}`,
            },
            body: JSON.stringify({
                email,
                global_attributes: {
                    firstname: firstName,
                    lastname: lastName,
                    anrede: salutation,
                },
                source: "",
            })
        }
    );
};
