<?php

/**
 * Plugin umami.
 */

use Shaarli\Config\ConfigManager;
use Shaarli\Plugin\PluginManager;

/**
 * @param array $data - header data.
 *
 * @return mixed
 */
function hook_umami_render_footer(array $data, ConfigManager $conf)
{
    $script_url = trim($conf->get('plugins.SCRIPT_URL', 'https://umami.is/umami.js'));
    $website_id = trim($conf->get('plugins.WEBSITE_ID', ''));
    $do_not_track = trim($conf->get('plugins.DO_NOT_TRACK', 'true'));
    $domains = trim($conf->get('plugins.DOMAINS', ''));

    if (!empty($website_id)) {
        $data['endofpage'][] = '<script async src="' . $script_url . '" data-website-id="' . $website_id . '" data-do-not-track="' . $do_not_track . '" data-domains="' . $domains . '"></script>';
    }

    return $data;
}

