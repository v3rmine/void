{ lib
, stdenv
, stdenvNoCC
, ostree 
, cacert
, zlib
, bzip2
, libopus
, udev
, dbus
, freetype
, libglvnd
, systemd
, libva
, libjpeg
, libdrm
, cairo
, harfbuzz
, icu69
, libICE
, libpng
, libvdpau
, glibc
, libX11
, openssl_1_1
, zstd
, pcre2
, glib
, libstdcxx5
, qt5
, fontconfig
, xorg
, libxkbcommon
, alsaOss
, pango
}:

let
  fetchFlatHub = 
    { url
    , rev ? ""
    , sha256 ? ""
    , postFetch ? ""
    }: stdenvNoCC.mkDerivation {
      inherit url rev;
      name = url;
      builder = ./flathub-builder.sh;
      fetcher = ./nix-prefetch-flathub.sh;
      nativeBuildInputs = [ ostree cacert ];
      outputHashAlgo = "sha256";
      outputHash = if sha256 != "" then
        sha256
      else
        lib.fakeSha256;
    };
  steamlinkSource = fetchFlatHub {
    url = "com.valvesoftware.SteamLink";
    rev = "82d065bac78309c4d97ca46ca9842f2f44803ab9ba6785b7aeeb56ab7ae9d728";
    sha256 = "sha256-fuAu5TusuZZjLn27jGFhw6v4bJa0XMZeutTO816/y4g=";
  };
  runtimeLibs = [
    cairo
    zlib
    bzip2
    libopus
    udev
    freetype
    libglvnd
    systemd
    libva
    libjpeg
    libpng
    libdrm
    cairo
    harfbuzz
    icu69
    libICE
    libvdpau
    libX11
    openssl_1_1
    zstd
    pcre2
    glib
    xorg.libxcb
    xorg.xcbutilwm
    xorg.xcbutilimage 
    xorg.xcbutilkeysyms
    xorg.xcbutilrenderutil
    xorg.libSM
    xorg.libXcomposite
    libxkbcommon
    pango
  ];
  rpath = builtins.foldl' (rpath: lib: rpath + ":${lib.out}/lib") "${dbus.lib}/lib:${fontconfig.lib}/lib" runtimeLibs; 
in qt5.mkDerivation rec {
  name = "steamlink";
  src = steamlinkSource;

  QT_PLUGIN_PATH="$out/lib/plugin";

  unpackPhase = ''
  mkdir -p $out
  tar -xf ${src} -C $out bin/ lib/ share/
  mkdir -p $out/share/licences
  tar -xf ${src} -C $out/share/licences LICENSE.txt ThirdPartyLegalNotices.css ThirdPartyLegalNotices.html
  '';
  installPhase = ''
  runHook preInstall
  chmod +x $out/lib/*.so*
  sed -i "s|/app|$out|" $out/share/applications/com.valvesoftware.SteamLink.desktop
  runHook postInstall
  '';
  preFixup = ''
  rpath="$out/lib:${rpath}"
  # echo ======================
  #   echo $rpath
  # echo ======================
  patchelf --set-rpath $rpath $out/bin/steamlink
  patchelf --set-rpath $rpath $(find $out/lib -name '*.so*')
  patchelf --set-interpreter ${glibc.out}/lib64/ld-linux-x86-64.so.2 $out/bin/steamlink
  patchelf --remove-needed libstdc++.so.6 $out/bin/steamlink
  patchelf --remove-needed libstdc++.so.6 $(find $out/lib -name '*.so*')
  '';
  fixupPhase = ''
  runHook preFixup
  runHook postFixup
  '';
  postFixup = ''
  mv $out/bin/steamlink $out/bin/steamlink-wrapped
  echo "#!/usr/bin/env bash" > $out/bin/steamlink
  echo "${alsaOss}/bin/aoss $out/bin/steamlink-wrapped" >> $out/bin/steamlink
  sed -i "s/exec -a \"\$0\"/exec -a \"\$0\" \"gdb\"/" $out/bin/steamlink-wrapped
  chmod +x $out/bin/steamlink
  '';

  buildInputs = [ glibc dbus fontconfig alsaOss ] ++ runtimeLibs;
  nativeBuildInputs = [ qt5.wrapQtAppsHook ];
}
