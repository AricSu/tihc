#/bin/bash
RED='\033[0;31m'
NC='\033[0m'
GREEN='\033[32m'
YELLOW='\033[33m'
BLUE='\033[36m'

echo -e "${GREEN}[Log        ] -->${YELLOW} WELCOME TO USE TiHC ... ... ${NC}"

tiup cluster display xu-tidb > ./deploy_grafana_image_render.log 2>/dev/null
PATH_GRAFANA_DEPLOY=`cat ./deploy_grafana_image_render.log |grep grafana |awk -F " " '{print $8}'`
IP_GRAFANA_DEPLOY=`cat ./deploy_grafana_image_render.log |grep grafana |awk -F " " '{print $3}'`



echo -e "${GREEN}[Log        ] -->${NC} Get grafana-image-renderer plugin in the grafana server !"
sleep 1

if [ ${2} == "online" ];then
  echo -e "${YELLOW}[Downloading] -->${NC} Get grafana-image-renderer plugin from Internet ..."
  echo -e "${BLUE}[Instruction] -->${NC} Please input passwd of root of ${IP_GRAFANA_DEPLOY} ,When prompted !"
  ssh -T root@${IP_GRAFANA_DEPLOY} ${PATH_GRAFANA_DEPLOY}/bin/bin/grafana-cli plugins install grafana-image-renderer
  PLUGIN_INSTALL=0
  echo -e "${BLUE}[Instruction] -->${NC} Please input passwd of root of ${IP_GRAFANA_DEPLOY} ,When prompted !"
  PLUGIN_INSTALL=`ssh -T root@${IP_GRAFANA_DEPLOY} ls /var/lib/grafana/plugins |grep grafana-image-renderer`
  echo ${PLUGIN_INSTALL}
  if [ ! ${PLUGIN_INSTALL} == "grafana-image-renderer"  ];then
    echo -e "${RED}[Error      ] --> If there output about 'Error: ✗ Failed to send request',you need to fix network issue,or use offline install !${NC}"
    exit
  fi

elif [ ${2} == "offline" ];then
  scp ${3} tidb@${IP_GRAFANA_DEPLOY}:${PATH_GRAFANA_DEPLOY}/ -i /home/tidb/.tiup/storage/cluster/clusters/${1}/ssh/id_rsa >/dev/null
  sleep 1
else
  echo -e "${RED}[ERROR      ] -->${NC} Please input right install way for grafana-image-renderer !"
  exit
fi


echo -e "${GREEN}[Log        ] -->${NC} Move grafana-image-renderer plugin package to tidb config path !"
if [ ${2} == "online" ];then
  echo -e "${BLUE}[Instruction] -->${NC} Please input passwd of root of ${IP_GRAFANA_DEPLOY} ,When prompted !"
  ssh  -T root@${IP_GRAFANA_DEPLOY}<< EOF
  cp -rf /var/lib/grafana/plugins/grafana-image-renderer ${PATH_GRAFANA_DEPLOY}/plugins/
  yum  -y install libXcomposite libXdamage libXtst cups libXScrnSaver pango atk adwaita-cursor-theme adwaita-icon-theme at at-spi2-atk at-spi2-core cairo-gobject colord-libs dconf desktop-file-utils ed emacs-filesystem gdk-pixbuf2 glib-networking gnutls gsettings-desktop-schemas gtk-update-icon-cache gtk3 hicolor-icon-theme jasper-libs json-glib libappindicator-gtk3 libdbusmenu libdbusmenu-gtk3 libepoxy liberation-fonts liberation-narrow-fonts liberation-sans-fonts liberation-serif-fonts libgusb libindicator-gtk3 libmodman libproxy libsoup libwayland-cursor libwayland-egl libxkbcommon m4 mailx nettle patch psmisc redhat-lsb-core redhat-lsb-submod-security rest spax time trousers xdg-utils xkeyboard-config alsa-lib >> /tmp/tihc_deploy_grafana_image_render.log 2>/dev/null
  chown tidb:tidb ${PATH_GRAFANA_DEPLOY}/plugins/grafana-image-renderer -R
EOF
elif [ ${2} == "offline" ];then
  scp -i /home/tidb/.tiup/storage/cluster/clusters/${1}/ssh/id_rsa ${3} tidb@${IP_GRAFANA_DEPLOY}:${PATH_GRAFANA_DEPLOY}/ >/dev/null
  echo -e "${BLUE}[Instruction] -->${NC} Please input passwd of root of ${IP_GRAFANA_DEPLOY} ,When prompted !"
  ssh  -T root@${IP_GRAFANA_DEPLOY}<< EOF
  tar -zxvf ${PATH_GRAFANA_DEPLOY}/grafana-image-renderer.tar.gz -C ${PATH_GRAFANA_DEPLOY}/plugins/ 1>/dev/null 2>/dev/null
  yum  -y install libXcomposite libXdamage libXtst cups libXScrnSaver pango atk adwaita-cursor-theme adwaita-icon-theme at at-spi2-atk at-spi2-core cairo-gobject colord-libs dconf desktop-file-utils ed emacs-filesystem gdk-pixbuf2 glib-networking gnutls gsettings-desktop-schemas gtk-update-icon-cache gtk3 hicolor-icon-theme jasper-libs json-glib libappindicator-gtk3 libdbusmenu libdbusmenu-gtk3 libepoxy liberation-fonts liberation-narrow-fonts liberation-sans-fonts liberation-serif-fonts libgusb libindicator-gtk3 libmodman libproxy libsoup libwayland-cursor libwayland-egl libxkbcommon m4 mailx nettle patch psmisc redhat-lsb-core redhat-lsb-submod-security rest spax time trousers xdg-utils xkeyboard-config alsa-lib >> /tmp/tihc_deploy_grafana_image_render.log 2>/dev/null
  chown tidb:tidb ${PATH_GRAFANA_DEPLOY}/plugins/grafana-image-renderer -R
EOF
else
  echo -e "${RED}[ERROR      ] -->${NC} Please input right install way for grafana-image-renderer !"
  exit
fi


echo -e "${GREEN}[Log        ] -->${NC} Config grafana-image-renderer for grafana server of tidb !"
ssh  -T -i /home/tidb/.tiup/storage/cluster/clusters/${1}/ssh/id_rsa ${IP_GRAFANA_DEPLOY} << EOF
echo "[plugins]" >> ${PATH_GRAFANA_DEPLOY}/conf/grafana.ini
echo "allow_loading_unsigned_plugins = grafana-image-renderer">> ${PATH_GRAFANA_DEPLOY}/conf/grafana.ini
echo "[plugin.grafana-image-renderer]">> ${PATH_GRAFANA_DEPLOY}/conf/grafana.ini
echo "rendering_ignore_https_errors = true">> ${PATH_GRAFANA_DEPLOY}/conf/grafana.ini
echo "rendering_verbose_logging = true">> ${PATH_GRAFANA_DEPLOY}/conf/grafana.ini
echo "rendering_args = --no-sandbox,--no-proxy-server,--disable-gpu">> ${PATH_GRAFANA_DEPLOY}/conf/grafana.ini
echo "[paths]">> ${PATH_GRAFANA_DEPLOY}/conf/grafana.ini
echo "temp_data_lifetime = 10m">> ${PATH_GRAFANA_DEPLOY}/conf/grafana.ini
cd ${PATH_GRAFANA_DEPLOY}

if [ ! -d plugins  ];then
  sudo mkdir plugins && cd plugins
else
  cd plugins
fi
EOF


echo -e "${GREEN}[Log        ] -->${YELLOW} grafana-image-renderer has been downloaded and configed successfully!${NC} !"
rm ./deploy_grafana_image_render.log
