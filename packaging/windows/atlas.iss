#define AppName "Dovy File Manager"
#ifndef AppVersion
  #define AppVersion "0.1.0"
#endif

[Setup]
AppId={{A66B9F93-8F34-4D8A-8D6A-A71A50000001}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher=Atlas
DefaultDirName={autopf}\Dovy File Manager
DefaultGroupName=Dovy File Manager
DisableProgramGroupPage=yes
OutputDir=..\..\target\installer
OutputBaseFilename=DovyFileManagerSetup-{#AppVersion}
Compression=lzma2
SolidCompression=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
SetupIconFile=..\icons\dovy.ico
UninstallDisplayIcon={app}\dovy.exe
ChangesEnvironment=yes
PrivilegesRequiredOverridesAllowed=dialog commandline

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "brazilianportuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"
Name: "french"; MessagesFile: "compiler:Languages\French.isl"
Name: "german"; MessagesFile: "compiler:Languages\German.isl"
Name: "italian"; MessagesFile: "compiler:Languages\Italian.isl"
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"
Name: "polish"; MessagesFile: "compiler:Languages\Polish.isl"
Name: "dutch"; MessagesFile: "compiler:Languages\Dutch.isl"
Name: "turkish"; MessagesFile: "compiler:Languages\Turkish.isl"
Name: "ukrainian"; MessagesFile: "compiler:Languages\Ukrainian.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Shortcuts:"
Name: "cli"; Description: "Enable the atlas command in Terminal"; GroupDescription: "Personalize Dovy File Manager:"
Name: "shell"; Description: "Add Open with Dovy File Manager to Explorer"; GroupDescription: "Personalize Dovy File Manager:"; Flags: checkedonce
Name: "searchservice"; Description: "Enable ultra-fast background search service (instant NTFS indexing)"; GroupDescription: "Personalize Dovy File Manager:"; Flags: checkedonce
Name: "launch"; Description: "Launch Dovy File Manager after install"; GroupDescription: "Finish:"; Flags: checkedonce

[Files]
Source: "..\..\target\release\dovy.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\target\release\dovy.exe"; DestDir: "{app}"; DestName: "atlas.exe"; Flags: ignoreversion
Source: "..\..\target\release\dovy-cloud-extension.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\target\release\dovy-cloud-extension.exe"; DestDir: "{app}"; DestName: "atlas-cloud-extension.exe"; Flags: ignoreversion
Source: "..\..\target\release\dovy-icon-extension.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\target\release\dovy-icon-extension.exe"; DestDir: "{app}"; DestName: "atlas-icon-extension.exe"; Flags: ignoreversion
Source: "..\..\legal\THIRD-PARTY-NOTICES.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE"; DestDir: "{app}"; DestName: "LICENSE.txt"; Flags: ignoreversion
Source: "..\..\crates\atlas-core\vendor\unrar-bin\UnRAR.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\target\extensions\cloud\rclone.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\target\extensions\cloud\THIRD-PARTY-NOTICES.txt"; DestDir: "{app}"; DestName: "CLOUD-THIRD-PARTY-NOTICES.txt"; Flags: ignoreversion
Source: "..\..\target\release\dovy-search-service.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\target\release\dovy-search-service.exe"; DestDir: "{app}"; DestName: "atlas-search-service.exe"; Flags: ignoreversion
Source: "..\icons\dovy.ico"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\vendor\gpui-component-fork\themes\*.json"; DestDir: "{userappdata}\dovy\themes"; Flags: ignoreversion
Source: "..\..\vendor\gpui-component-fork\themes\*.json"; DestDir: "{userappdata}\atlas\themes"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Dovy File Manager"; Filename: "{app}\dovy.exe"; IconFilename: "{app}\dovy.ico"
Name: "{autodesktop}\Dovy File Manager"; Filename: "{app}\dovy.exe"; IconFilename: "{app}\dovy.ico"; Tasks: desktopicon

[Registry]
Root: HKCU; Subkey: "Software\Classes\Directory\shell\DovyFileManager"; ValueType: string; ValueName: ""; ValueData: "Open with Dovy File Manager"; Tasks: shell; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Directory\shell\DovyFileManager"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\dovy.ico"; Tasks: shell
Root: HKCU; Subkey: "Software\Classes\Directory\shell\DovyFileManager\command"; ValueType: string; ValueName: ""; ValueData: """{app}\dovy.exe"" ""%1"""; Tasks: shell
Root: HKCU; Subkey: "Software\Classes\Directory\Background\shell\DovyFileManager"; ValueType: string; ValueName: ""; ValueData: "Open with Dovy File Manager"; Tasks: shell; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Directory\Background\shell\DovyFileManager"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\dovy.ico"; Tasks: shell
Root: HKCU; Subkey: "Software\Classes\Directory\Background\shell\DovyFileManager\command"; ValueType: string; ValueName: ""; ValueData: """{app}\dovy.exe"" ""%V"""; Tasks: shell
Root: HKCU; Subkey: "Software\Classes\Applications\dovy.exe"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "Dovy File Manager"; Tasks: shell; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Applications\dovy.exe\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\dovy.exe"" ""%1"""; Tasks: shell
Root: HKCU; Subkey: "Software\Classes\Applications\atlas.exe"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "Dovy File Manager"; Tasks: shell; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Applications\atlas.exe\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\dovy.exe"" ""%1"""; Tasks: shell

[Run]
Filename: "{app}\dovy-search-service.exe"; Parameters: "--install"; Flags: runhidden; Tasks: searchservice; StatusMsg: "Configuring Dovy File Manager Search Service..."; Check: IsAdminInstallMode
Filename: "{app}\dovy.exe"; Description: "Launch Dovy File Manager"; Flags: nowait postinstall skipifsilent; Tasks: launch

[UninstallRun]
Filename: "{app}\dovy-search-service.exe"; Parameters: "--uninstall"; Flags: runhidden; Check: IsAdminInstallMode
Filename: "{app}\atlas-search-service.exe"; Parameters: "--uninstall"; Flags: runhidden; Check: IsAdminInstallMode


[Code]
const
  EnvKey = 'Environment';

function PathPartExists(PathValue, Dir: String): Boolean;
begin
  Result := Pos(';' + Uppercase(Dir) + ';', ';' + Uppercase(PathValue) + ';') > 0;
end;

procedure AppendUserPath(Dir: String);
var
  PathValue: String;
begin
  if not RegQueryStringValue(HKCU, EnvKey, 'Path', PathValue) then
    PathValue := '';

  if PathPartExists(PathValue, Dir) then
    exit;

  if PathValue = '' then
    RegWriteStringValue(HKCU, EnvKey, 'Path', Dir)
  else
    RegWriteStringValue(HKCU, EnvKey, 'Path', PathValue + ';' + Dir);
end;

procedure RemoveUserPath(Dir: String);
var
  PathValue: String;
begin
  if not RegQueryStringValue(HKCU, EnvKey, 'Path', PathValue) then
    exit;

  StringChangeEx(PathValue, ';' + Dir + ';', ';', True);
  StringChangeEx(PathValue, Dir + ';', '', True);
  StringChangeEx(PathValue, ';' + Dir, '', True);

  if PathValue = Dir then
    PathValue := '';

  RegWriteStringValue(HKCU, EnvKey, 'Path', PathValue);
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if (CurStep = ssPostInstall) and WizardIsTaskSelected('cli') then
    AppendUserPath(ExpandConstant('{app}'));
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
    RemoveUserPath(ExpandConstant('{app}'));
end;
