; Kaptura Profesyonel Kurulum Scripti (Inno Setup)

[Setup]
; AppId değeri her sürümde sabit kalmalıdır (Update yönetimi için)
AppId={{C6D2A8B1-F3E4-4A8E-B9D1-F2E3A4B5C6D7}
AppName=Kaptura
AppVersion=1.2.0
AppPublisher=Eser Şahiner
AppPublisherURL=https://github.com/esershnr/kaptura
AppSupportURL=https://github.com/esershnr/kaptura
AppUpdatesURL=https://github.com/esershnr/kaptura
DefaultDirName={autopf}\Kaptura
DefaultGroupName=Kaptura
UninstallDisplayIcon={app}\kaptura.exe
Compression=lzma2
SolidCompression=yes
OutputDir=Output
OutputBaseFilename=Kaptura_Setup_v1.1.0
SetupIconFile=assets\icon.ico
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
; Her şeyi senin hazırladığın dist klasöründen alıyoruz
Source: "dist\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{group}\Kaptura"; Filename: "{app}\kaptura.exe"
Name: "{autodesktop}\Kaptura"; Filename: "{app}\kaptura.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\kaptura.exe"; Description: "{cm:LaunchProgram,Kaptura}"; Flags: nowait postinstall skipifsilent
