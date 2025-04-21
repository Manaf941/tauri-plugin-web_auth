# Tauri Plugin web_auth
> [!CAUTION]
> This was a plugin primarily made for myself. I've made it open-source so that other people could use it, but I'm not willing to document everything. If you do need some help / clarification / changes, you can contact me on Discord / Twitter: `manaf941` / `manaaaaaaaf`

> [!WARNING]
> I have NOT tested this plugin in conjunction with the Tauri Deep Linking plugin. It MAY be incompatible. I cannot guarantee this will work with your setup.

> [!INFO]
> This plugin only works on iOS and Android. I have no plan to bring it to Desktop. I will answer pull requests though

# Demo
\[future me insert video here]

# Usage
## `src-tauri/src/lib.rs`
Add crate tauri-plugin-web-auth first
```rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // this line initializes the plugin
        .plugin(tauri_plugin_web_auth::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## `src-tauri/capabilities/default.json`
```json
{
    "permissions": [
        "web-auth:default",
        "web-auth:allow-authenticate"
    ]
}

```

## `index.ts`
```ts
import { authenticate } from 'tauri-plugin-web-auth-api'

// Example: Authenticate to google
// scheme is `iOS URL scheme` in Google Cloud Console
const scheme = "com.googleusercontent.apps.807645982538-7ib8rr1tkjkeb3vr0u1lk7gs6ip3b0jl"
const url = new URL("https://accounts.google.com/o/oauth2/v2/auth")
url.searchParams.set("response_type", "code")
// client id is `Client ID` under Google Cloud Console, it's basically the reverse scheme
url.searchParams.set("client_id", "807645982538-7ib8rr1tkjkeb3vr0u1lk7gs6ip3b0jl.apps.googleusercontent.com")
url.searchParams.set("redirect_uri", scheme+":/")
// scope must be separated by spaces
url.searchParams.set("scope", "email")
s
const res = await authenticate({
    url: url.toString(),
    callbackScheme: scheme,
})

const callback_url = new URL(res.callbackUrl)
// You can then exchange this code with your backend or with google to
// get the user's informations
console.log(`Got code: ${callback_url.searchParams.get("code")}`)   
```

# Setup
## iOS
iOS should work out of the box, no need to modify any configuration files.

## Android
Android is bit more annoying. You need to add the callback scheme to your `src-tauri/gen/android/app/src/AndroidManifest.xml` file.
```xml
<!-- Find the android:name=".MainActivitiy" Activity and add the second intent filter-->
<activity
    android:configChanges="orientation|keyboardHidden|keyboard|screenSize|locale|smallestScreenSize|screenLayout|uiMode"
    android:launchMode="singleTask"
    android:label="@string/main_activity_title"
    android:name=".MainActivity"
    android:exported="true">
    <intent-filter>
        ...
    </intent-filter>

    <!-- Add this intent filter to allow Google Authentication -->
    <!-- You need to add one intent filter per scheme -->
    <intent-filter>
        <action android:name="android.intent.action.VIEW" />
        <category android:name="android.intent.category.DEFAULT" />
        <category android:name="android.intent.category.BROWSABLE" />
        <data android:scheme="com.googleusercontent.apps.807645982538-7ib8rr1tkjkeb3vr0u1lk7gs6ip3b0jl" />
    </intent-filter>
</activity>
```

# Contact
In case of questions, contact me on my socials:
Discord: @manaf941
Twitter: manaaaaaaaf
