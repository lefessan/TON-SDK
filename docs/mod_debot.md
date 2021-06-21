# Module debot

[UNSTABLE](UNSTABLE.md) Module for working with debot.


## Functions
[init](#init) – [UNSTABLE](UNSTABLE.md) Creates and instance of DeBot.

[start](#start) – [UNSTABLE](UNSTABLE.md) Starts the DeBot.

[fetch](#fetch) – [UNSTABLE](UNSTABLE.md) Fetches DeBot metadata from blockchain.

[execute](#execute) – [UNSTABLE](UNSTABLE.md) Executes debot action.

[send](#send) – [UNSTABLE](UNSTABLE.md) Sends message to Debot.

[remove](#remove) – [UNSTABLE](UNSTABLE.md) Destroys debot handle.

## Types
[DebotErrorCode](#DebotErrorCode)

[DebotHandle](#DebotHandle) – [UNSTABLE](UNSTABLE.md) Handle of registered in SDK debot

[DebotAction](#DebotAction) – [UNSTABLE](UNSTABLE.md) Describes a debot action in a Debot Context.

[DebotInfo](#DebotInfo) – [UNSTABLE](UNSTABLE.md) Describes DeBot metadata.

[DebotActivity](#DebotActivity) – [UNSTABLE](UNSTABLE.md) Describes the operation that the DeBot wants to perform.

[Spending](#Spending) – [UNSTABLE](UNSTABLE.md) Describes how much funds will be debited from the target  contract balance as a result of the transaction.

[ParamsOfInit](#ParamsOfInit) – [UNSTABLE](UNSTABLE.md) Parameters to init DeBot.

[RegisteredDebot](#RegisteredDebot) – [UNSTABLE](UNSTABLE.md) Structure for storing debot handle returned from `init` function.

[ParamsOfAppDebotBrowser](#ParamsOfAppDebotBrowser) – [UNSTABLE](UNSTABLE.md) Debot Browser callbacks

[ResultOfAppDebotBrowser](#ResultOfAppDebotBrowser) – [UNSTABLE](UNSTABLE.md) Returning values from Debot Browser callbacks.

[ParamsOfStart](#ParamsOfStart) – [UNSTABLE](UNSTABLE.md) Parameters to start DeBot. DeBot must be already initialized with init() function.

[ParamsOfFetch](#ParamsOfFetch) – [UNSTABLE](UNSTABLE.md) Parameters to fetch DeBot metadata.

[ResultOfFetch](#ResultOfFetch) – [UNSTABLE](UNSTABLE.md)

[ParamsOfExecute](#ParamsOfExecute) – [UNSTABLE](UNSTABLE.md) Parameters for executing debot action.

[ParamsOfSend](#ParamsOfSend) – [UNSTABLE](UNSTABLE.md) Parameters of `send` function.

[ParamsOfRemove](#ParamsOfRemove) – [UNSTABLE](UNSTABLE.md)

[AppDebotBrowser](#AppDebotBrowser)


# Functions
## init

[UNSTABLE](UNSTABLE.md) Creates and instance of DeBot.

Downloads debot smart contract (code and data) from blockchain and creates
an instance of Debot Engine for it.

# Remarks
It does not switch debot to context 0. Browser Callbacks are not called.

```ts
type ParamsOfInit = {
    address: string
}

type RegisteredDebot = {
    debot_handle: DebotHandle,
    debot_abi: string,
    info: DebotInfo
}

function init(
    params: ParamsOfInit,
    obj: AppDebotBrowser,
): Promise<RegisteredDebot>;
```
### Parameters
- `address`: _string_ – Debot smart contract address


### Result

- `debot_handle`: _[DebotHandle](mod_debot.md#DebotHandle)_ – Debot handle which references an instance of debot engine.
- `debot_abi`: _string_ – Debot abi as json string.
- `info`: _[DebotInfo](mod_debot.md#DebotInfo)_ – Debot metadata.


## start

[UNSTABLE](UNSTABLE.md) Starts the DeBot.

Downloads debot smart contract from blockchain and switches it to
context zero.

This function must be used by Debot Browser to start a dialog with debot.
While the function is executing, several Browser Callbacks can be called,
since the debot tries to display all actions from the context 0 to the user.

When the debot starts SDK registers `BrowserCallbacks` AppObject.
Therefore when `debote.remove` is called the debot is being deleted and the callback is called
with `finish`=`true` which indicates that it will never be used again.

```ts
type ParamsOfStart = {
    debot_handle: DebotHandle
}

function start(
    params: ParamsOfStart,
): Promise<void>;
```
### Parameters
- `debot_handle`: _[DebotHandle](mod_debot.md#DebotHandle)_ – Debot handle which references an instance of debot engine.


## fetch

[UNSTABLE](UNSTABLE.md) Fetches DeBot metadata from blockchain.

Downloads DeBot from blockchain and creates and fetches its metadata.

```ts
type ParamsOfFetch = {
    address: string
}

type ResultOfFetch = {
    info: DebotInfo
}

function fetch(
    params: ParamsOfFetch,
): Promise<ResultOfFetch>;
```
### Parameters
- `address`: _string_ – Debot smart contract address.


### Result

- `info`: _[DebotInfo](mod_debot.md#DebotInfo)_ – Debot metadata.


## execute

[UNSTABLE](UNSTABLE.md) Executes debot action.

Calls debot engine referenced by debot handle to execute input action.
Calls Debot Browser Callbacks if needed.

# Remarks
Chain of actions can be executed if input action generates a list of subactions.

```ts
type ParamsOfExecute = {
    debot_handle: DebotHandle,
    action: DebotAction
}

function execute(
    params: ParamsOfExecute,
): Promise<void>;
```
### Parameters
- `debot_handle`: _[DebotHandle](mod_debot.md#DebotHandle)_ – Debot handle which references an instance of debot engine.
- `action`: _[DebotAction](mod_debot.md#DebotAction)_ – Debot Action that must be executed.


## send

[UNSTABLE](UNSTABLE.md) Sends message to Debot.

Used by Debot Browser to send response on Dinterface call or from other Debots.

```ts
type ParamsOfSend = {
    debot_handle: DebotHandle,
    message: string
}

function send(
    params: ParamsOfSend,
): Promise<void>;
```
### Parameters
- `debot_handle`: _[DebotHandle](mod_debot.md#DebotHandle)_ – Debot handle which references an instance of debot engine.
- `message`: _string_ – BOC of internal message to debot encoded in base64 format.


## remove

[UNSTABLE](UNSTABLE.md) Destroys debot handle.

Removes handle from Client Context and drops debot engine referenced by that handle.

```ts
type ParamsOfRemove = {
    debot_handle: DebotHandle
}

function remove(
    params: ParamsOfRemove,
): Promise<void>;
```
### Parameters
- `debot_handle`: _[DebotHandle](mod_debot.md#DebotHandle)_ – Debot handle which references an instance of debot engine.


# Types
## DebotErrorCode
```ts
enum DebotErrorCode {
    DebotStartFailed = 801,
    DebotFetchFailed = 802,
    DebotExecutionFailed = 803,
    DebotInvalidHandle = 804,
    DebotInvalidJsonParams = 805,
    DebotInvalidFunctionId = 806,
    DebotInvalidAbi = 807,
    DebotGetMethodFailed = 808,
    DebotInvalidMsg = 809,
    DebotExternalCallFailed = 810,
    DebotBrowserCallbackFailed = 811,
    DebotOperationRejected = 812
}
```
One of the following value:

- `DebotStartFailed = 801`
- `DebotFetchFailed = 802`
- `DebotExecutionFailed = 803`
- `DebotInvalidHandle = 804`
- `DebotInvalidJsonParams = 805`
- `DebotInvalidFunctionId = 806`
- `DebotInvalidAbi = 807`
- `DebotGetMethodFailed = 808`
- `DebotInvalidMsg = 809`
- `DebotExternalCallFailed = 810`
- `DebotBrowserCallbackFailed = 811`
- `DebotOperationRejected = 812`


## DebotHandle
[UNSTABLE](UNSTABLE.md) Handle of registered in SDK debot

```ts
type DebotHandle = number
```


## DebotAction
[UNSTABLE](UNSTABLE.md) Describes a debot action in a Debot Context.

```ts
type DebotAction = {
    description: string,
    name: string,
    action_type: number,
    to: number,
    attributes: string,
    misc: string
}
```
- `description`: _string_ – A short action description.
<br>Should be used by Debot Browser as name of menu item.
- `name`: _string_ – Depends on action type.
<br>Can be a debot function name or a print string (for Print Action).
- `action_type`: _number_ – Action type.
- `to`: _number_ – ID of debot context to switch after action execution.
- `attributes`: _string_ – Action attributes.
<br>In the form of "param=value,flag". attribute example: instant, args, fargs, sign.
- `misc`: _string_ – Some internal action data.
<br>Used by debot only.


## DebotInfo
[UNSTABLE](UNSTABLE.md) Describes DeBot metadata.

```ts
type DebotInfo = {
    name?: string,
    version?: string,
    publisher?: string,
    caption?: string,
    author?: string,
    support?: string,
    hello?: string,
    language?: string,
    dabi?: string,
    icon?: string,
    interfaces: string[]
}
```
- `name`?: _string_ – DeBot short name.
- `version`?: _string_ – DeBot semantic version.
- `publisher`?: _string_ – The name of DeBot deployer.
- `caption`?: _string_ – Short info about DeBot.
- `author`?: _string_ – The name of DeBot developer.
- `support`?: _string_ – TON address of author for questions and donations.
- `hello`?: _string_ – String with the first messsage from DeBot.
- `language`?: _string_ – String with DeBot interface language (ISO-639).
- `dabi`?: _string_ – String with DeBot ABI.
- `icon`?: _string_ – DeBot icon.
- `interfaces`: _string[]_ – Vector with IDs of DInterfaces used by DeBot.


## DebotActivity
[UNSTABLE](UNSTABLE.md) Describes the operation that the DeBot wants to perform.

```ts
type DebotActivity = {
    type: 'Transaction'
    msg: string,
    dst: string,
    out: Spending[],
    fee: bigint,
    setcode: boolean,
    signkey: string,
    signing_box_handle: number
}
```
Depends on value of the  `type` field.

When _type_ is _'Transaction'_

DeBot wants to create new transaction in blockchain.


- `msg`: _string_ – External inbound message BOC.
- `dst`: _string_ – Target smart contract address.
- `out`: _[Spending](mod_debot.md#Spending)[]_ – List of spendings as a result of transaction.
- `fee`: _bigint_ – Transaction total fee.
- `setcode`: _boolean_ – Indicates if target smart contract updates its code.
- `signkey`: _string_ – Public key from keypair that was used to sign external message.
- `signing_box_handle`: _number_ – Signing box handle used to sign external message.


Variant constructors:

```ts
function debotActivityTransaction(msg: string, dst: string, out: Spending[], fee: bigint, setcode: boolean, signkey: string, signing_box_handle: number): DebotActivity;
```

## Spending
[UNSTABLE](UNSTABLE.md) Describes how much funds will be debited from the target  contract balance as a result of the transaction.

```ts
type Spending = {
    amount: bigint,
    dst: string
}
```
- `amount`: _bigint_ – Amount of nanotokens that will be sent to `dst` address.
- `dst`: _string_ – Destination address of recipient of funds.


## ParamsOfInit
[UNSTABLE](UNSTABLE.md) Parameters to init DeBot.

```ts
type ParamsOfInit = {
    address: string
}
```
- `address`: _string_ – Debot smart contract address


## RegisteredDebot
[UNSTABLE](UNSTABLE.md) Structure for storing debot handle returned from `init` function.

```ts
type RegisteredDebot = {
    debot_handle: DebotHandle,
    debot_abi: string,
    info: DebotInfo
}
```
- `debot_handle`: _[DebotHandle](mod_debot.md#DebotHandle)_ – Debot handle which references an instance of debot engine.
- `debot_abi`: _string_ – Debot abi as json string.
- `info`: _[DebotInfo](mod_debot.md#DebotInfo)_ – Debot metadata.


## ParamsOfAppDebotBrowser
[UNSTABLE](UNSTABLE.md) Debot Browser callbacks

Called by debot engine to communicate with debot browser.

```ts
type ParamsOfAppDebotBrowser = {
    type: 'Log'
    msg: string
} | {
    type: 'Switch'
    context_id: number
} | {
    type: 'SwitchCompleted'
} | {
    type: 'ShowAction'
    action: DebotAction
} | {
    type: 'Input'
    prompt: string
} | {
    type: 'GetSigningBox'
} | {
    type: 'InvokeDebot'
    debot_addr: string,
    action: DebotAction
} | {
    type: 'Send'
    message: string
} | {
    type: 'Approve'
    activity: DebotActivity
}
```
Depends on value of the  `type` field.

When _type_ is _'Log'_

Print message to user.


- `msg`: _string_ – A string that must be printed to user.

When _type_ is _'Switch'_

Switch debot to another context (menu).


- `context_id`: _number_ – Debot context ID to which debot is switched.

When _type_ is _'SwitchCompleted'_

Notify browser that all context actions are shown.


When _type_ is _'ShowAction'_

Show action to the user. Called after `switch` for each action in context.


- `action`: _[DebotAction](mod_debot.md#DebotAction)_ – Debot action that must be shown to user as menu item. At least `description` property must be shown from [DebotAction] structure.

When _type_ is _'Input'_

Request user input.


- `prompt`: _string_ – A prompt string that must be printed to user before input request.

When _type_ is _'GetSigningBox'_

Get signing box to sign data.

Signing box returned is owned and disposed by debot engine


When _type_ is _'InvokeDebot'_

Execute action of another debot.


- `debot_addr`: _string_ – Address of debot in blockchain.
- `action`: _[DebotAction](mod_debot.md#DebotAction)_ – Debot action to execute.

When _type_ is _'Send'_

Used by Debot to call DInterface implemented by Debot Browser.


- `message`: _string_ – Internal message to DInterface address.
<br>Message body contains interface function and parameters.

When _type_ is _'Approve'_

Requests permission from DeBot Browser to execute DeBot operation.


- `activity`: _[DebotActivity](mod_debot.md#DebotActivity)_ – DeBot activity details.


Variant constructors:

```ts
function paramsOfAppDebotBrowserLog(msg: string): ParamsOfAppDebotBrowser;
function paramsOfAppDebotBrowserSwitch(context_id: number): ParamsOfAppDebotBrowser;
function paramsOfAppDebotBrowserSwitchCompleted(): ParamsOfAppDebotBrowser;
function paramsOfAppDebotBrowserShowAction(action: DebotAction): ParamsOfAppDebotBrowser;
function paramsOfAppDebotBrowserInput(prompt: string): ParamsOfAppDebotBrowser;
function paramsOfAppDebotBrowserGetSigningBox(): ParamsOfAppDebotBrowser;
function paramsOfAppDebotBrowserInvokeDebot(debot_addr: string, action: DebotAction): ParamsOfAppDebotBrowser;
function paramsOfAppDebotBrowserSend(message: string): ParamsOfAppDebotBrowser;
function paramsOfAppDebotBrowserApprove(activity: DebotActivity): ParamsOfAppDebotBrowser;
```

## ResultOfAppDebotBrowser
[UNSTABLE](UNSTABLE.md) Returning values from Debot Browser callbacks.

```ts
type ResultOfAppDebotBrowser = {
    type: 'Input'
    value: string
} | {
    type: 'GetSigningBox'
    signing_box: SigningBoxHandle
} | {
    type: 'InvokeDebot'
} | {
    type: 'Approve'
    approved: boolean
}
```
Depends on value of the  `type` field.

When _type_ is _'Input'_

Result of user input.


- `value`: _string_ – String entered by user.

When _type_ is _'GetSigningBox'_

Result of getting signing box.


- `signing_box`: _[SigningBoxHandle](mod_crypto.md#SigningBoxHandle)_ – Signing box for signing data requested by debot engine.
<br>Signing box is owned and disposed by debot engine

When _type_ is _'InvokeDebot'_

Result of debot invoking.


When _type_ is _'Approve'_

Result of `approve` callback.


- `approved`: _boolean_ – Indicates whether the DeBot is allowed to perform the specified operation.


Variant constructors:

```ts
function resultOfAppDebotBrowserInput(value: string): ResultOfAppDebotBrowser;
function resultOfAppDebotBrowserGetSigningBox(signing_box: SigningBoxHandle): ResultOfAppDebotBrowser;
function resultOfAppDebotBrowserInvokeDebot(): ResultOfAppDebotBrowser;
function resultOfAppDebotBrowserApprove(approved: boolean): ResultOfAppDebotBrowser;
```

## ParamsOfStart
[UNSTABLE](UNSTABLE.md) Parameters to start DeBot. DeBot must be already initialized with init() function.

```ts
type ParamsOfStart = {
    debot_handle: DebotHandle
}
```
- `debot_handle`: _[DebotHandle](mod_debot.md#DebotHandle)_ – Debot handle which references an instance of debot engine.


## ParamsOfFetch
[UNSTABLE](UNSTABLE.md) Parameters to fetch DeBot metadata.

```ts
type ParamsOfFetch = {
    address: string
}
```
- `address`: _string_ – Debot smart contract address.


## ResultOfFetch
[UNSTABLE](UNSTABLE.md)

```ts
type ResultOfFetch = {
    info: DebotInfo
}
```
- `info`: _[DebotInfo](mod_debot.md#DebotInfo)_ – Debot metadata.


## ParamsOfExecute
[UNSTABLE](UNSTABLE.md) Parameters for executing debot action.

```ts
type ParamsOfExecute = {
    debot_handle: DebotHandle,
    action: DebotAction
}
```
- `debot_handle`: _[DebotHandle](mod_debot.md#DebotHandle)_ – Debot handle which references an instance of debot engine.
- `action`: _[DebotAction](mod_debot.md#DebotAction)_ – Debot Action that must be executed.


## ParamsOfSend
[UNSTABLE](UNSTABLE.md) Parameters of `send` function.

```ts
type ParamsOfSend = {
    debot_handle: DebotHandle,
    message: string
}
```
- `debot_handle`: _[DebotHandle](mod_debot.md#DebotHandle)_ – Debot handle which references an instance of debot engine.
- `message`: _string_ – BOC of internal message to debot encoded in base64 format.


## ParamsOfRemove
[UNSTABLE](UNSTABLE.md)

```ts
type ParamsOfRemove = {
    debot_handle: DebotHandle
}
```
- `debot_handle`: _[DebotHandle](mod_debot.md#DebotHandle)_ – Debot handle which references an instance of debot engine.


## AppDebotBrowser

```ts

type ParamsOfAppDebotBrowserLog = {
    msg: string
}

type ParamsOfAppDebotBrowserSwitch = {
    context_id: number
}

type ParamsOfAppDebotBrowserShowAction = {
    action: DebotAction
}

type ParamsOfAppDebotBrowserInput = {
    prompt: string
}

type ResultOfAppDebotBrowserInput = {
    value: string
}

type ResultOfAppDebotBrowserGetSigningBox = {
    signing_box: SigningBoxHandle
}

type ParamsOfAppDebotBrowserInvokeDebot = {
    debot_addr: string,
    action: DebotAction
}

type ParamsOfAppDebotBrowserSend = {
    message: string
}

type ParamsOfAppDebotBrowserApprove = {
    activity: DebotActivity
}

type ResultOfAppDebotBrowserApprove = {
    approved: boolean
}

export interface AppDebotBrowser {
    log(params: ParamsOfAppDebotBrowserLog): void,
    switch(params: ParamsOfAppDebotBrowserSwitch): void,
    switch_completed(): void,
    show_action(params: ParamsOfAppDebotBrowserShowAction): void,
    input(params: ParamsOfAppDebotBrowserInput): Promise<ResultOfAppDebotBrowserInput>,
    get_signing_box(): Promise<ResultOfAppDebotBrowserGetSigningBox>,
    invoke_debot(params: ParamsOfAppDebotBrowserInvokeDebot): Promise<void>,
    send(params: ParamsOfAppDebotBrowserSend): void,
    approve(params: ParamsOfAppDebotBrowserApprove): Promise<ResultOfAppDebotBrowserApprove>,
}
```

## log

Print message to user.

```ts
type ParamsOfAppDebotBrowserLog = {
    msg: string
}

function log(
    params: ParamsOfAppDebotBrowserLog,
): Promise<>;
```
### Parameters
- `msg`: _string_ – A string that must be printed to user.


## switch

Switch debot to another context (menu).

```ts
type ParamsOfAppDebotBrowserSwitch = {
    context_id: number
}

function switch(
    params: ParamsOfAppDebotBrowserSwitch,
): Promise<>;
```
### Parameters
- `context_id`: _number_ – Debot context ID to which debot is switched.


## switch_completed

Notify browser that all context actions are shown.

```ts
function switch_completed(): Promise<>;
```


## show_action

Show action to the user. Called after `switch` for each action in context.

```ts
type ParamsOfAppDebotBrowserShowAction = {
    action: DebotAction
}

function show_action(
    params: ParamsOfAppDebotBrowserShowAction,
): Promise<>;
```
### Parameters
- `action`: _[DebotAction](mod_debot.md#DebotAction)_ – Debot action that must be shown to user as menu item. At least `description` property must be shown from [DebotAction] structure.


## input

Request user input.

```ts
type ParamsOfAppDebotBrowserInput = {
    prompt: string
}

type ResultOfAppDebotBrowserInput = {
    value: string
}

function input(
    params: ParamsOfAppDebotBrowserInput,
): Promise<ResultOfAppDebotBrowserInput>;
```
### Parameters
- `prompt`: _string_ – A prompt string that must be printed to user before input request.


### Result

- `value`: _string_ – String entered by user.


## get_signing_box

Get signing box to sign data.

Signing box returned is owned and disposed by debot engine

```ts
type ResultOfAppDebotBrowserGetSigningBox = {
    signing_box: SigningBoxHandle
}

function get_signing_box(): Promise<ResultOfAppDebotBrowserGetSigningBox>;
```


### Result

- `signing_box`: _[SigningBoxHandle](mod_crypto.md#SigningBoxHandle)_ – Signing box for signing data requested by debot engine.
<br>Signing box is owned and disposed by debot engine


## invoke_debot

Execute action of another debot.

```ts
type ParamsOfAppDebotBrowserInvokeDebot = {
    debot_addr: string,
    action: DebotAction
}

function invoke_debot(
    params: ParamsOfAppDebotBrowserInvokeDebot,
): Promise<void>;
```
### Parameters
- `debot_addr`: _string_ – Address of debot in blockchain.
- `action`: _[DebotAction](mod_debot.md#DebotAction)_ – Debot action to execute.


## send

Used by Debot to call DInterface implemented by Debot Browser.

```ts
type ParamsOfAppDebotBrowserSend = {
    message: string
}

function send(
    params: ParamsOfAppDebotBrowserSend,
): Promise<>;
```
### Parameters
- `message`: _string_ – Internal message to DInterface address.
<br>Message body contains interface function and parameters.


## approve

Requests permission from DeBot Browser to execute DeBot operation.

```ts
type ParamsOfAppDebotBrowserApprove = {
    activity: DebotActivity
}

type ResultOfAppDebotBrowserApprove = {
    approved: boolean
}

function approve(
    params: ParamsOfAppDebotBrowserApprove,
): Promise<ResultOfAppDebotBrowserApprove>;
```
### Parameters
- `activity`: _[DebotActivity](mod_debot.md#DebotActivity)_ – DeBot activity details.


### Result

- `approved`: _boolean_ – Indicates whether the DeBot is allowed to perform the specified operation.


