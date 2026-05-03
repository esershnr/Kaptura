; Kaptura Profesyonel Kurulum Scripti (Inno Setup)

[Setup]
AppName=Kaptura
AppVersion=1.0
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
OutputBaseFilename=Kaptura_Setup_v1.0
SetupIconFile=assets\icon.ico

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
