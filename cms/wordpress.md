# Wordpress Study Notes

## Hooks

- register_activation_hook(__FILE__, 'pluginprefix_install')

## Tips

### Debug

- wp-config.php
- plugins
- var_export + file_put_content: make sure the output file is writable

### Setup with Nginx in front

#### nginx file

```
location / {
	if ($host !~* ^www) {
		set $name_www www.$host;
		rewrite ^(.*) https://$name_www$1 permanent;
	}
	proxy_pass  http://localhost:8000;
	proxy_redirect     off;
	proxy_set_header   Host $host;
	proxy_set_header   X-Real-IP $remote_addr;
	proxy_set_header   X-Forwarded-For $proxy_add_x_forwarded_for;
	proxy_set_header   X-Forwarded-Host $server_name;
	proxy_set_header   X-Forwarded-Proto https;
	proxy_set_header   Upgrade $http_upgrade;
	proxy_set_header   Connection "upgrade";
	proxy_read_timeout 86400;
}
```

#### Add below to top of wp-config.php

```
define('FORCE_SSL_ADMIN', true);

if (strpos($_SERVER['HTTP_X_FORWARDED_PROTO'], 'https') !== false){
    $_SERVER['HTTPS'] = 'on';
    $_SERVER['SERVER_PORT'] = 443;
}
if (isset($_SERVER['HTTP_X_FORWARDED_HOST'])) {
    $_SERVER['HTTP_HOST'] = $_SERVER['HTTP_X_FORWARDED_HOST'];
}

define('WP_HOME','https://www.domainname.cc/');
define('WP_SITEURL','https://www.domainname.cc/');
```

## References

- [WordPress 啟用、停用、刪除外掛](https://ithelp.ithome.com.tw/articles/10234412?sc=iThomeR) 
- [Debugging in WordPress](https://wordpress.org/documentation/article/debugging-in-wordpress/#)

## Plugins

## Books 

- Building Web Apps with WordPress: WordPress as an Application Framework
- WordPress 4.x Complete
- Professional WordPress Design and Development, 3rd Edition

