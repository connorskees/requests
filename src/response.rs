use std::time::Duration;

pub struct Response {
    reponse_status: u8,
    url: String,
    encoding: String,
    reason: String,
    elapsed: Duration,
    content: String,
    bytes: [u8]
}
//  'url': 'http://www.google.com/'
//  'encoding': 'ISO-8859-1'
//  'history': [<Response [301]>]
//  'reason': 'OK'
//  'cookies': <RequestsCookieJar[Cookie(version=0, name='1P_JAR', value='2019-06-25-14', port=None, port_specified=False, domain='.google.com', domain_specified=True, domain_initial_dot=True, path='/', path_specified=True, secure=False, expires=1564065251, discard=False, comment=None, comment_url=None, rest={}, rfc2109=False), Cookie(version=0, name='NID', value='186=CpSsi0YmUw6bzJYvWvzJo3MNKNqF8sFM6TecUFUTl7KpyvbOq7YVFtYHxwMcTcbolOeE15jiAETP7UtzX8iavoV0240W03h6sA1APKkGQq4GuQRjhPq-pbSNJ7bdx8uHl1Wmt7Z8IOPChiXxHR8JOI904mD09pfcG8O4Q77yvhs', port=None, port_specified=False, domain='.google.com', domain_specified=True, domain_initial_dot=True, path='/', path_specified=True, secure=False, expires=1577284451, discard=False, comment=None, comment_url=None, rest={'HttpOnly': None}, rfc2109=False)]>,
//  'elapsed': datetime.timedelta(seconds=3, microseconds=76572),
//  'request': <PreparedRequest [GET]>,
//  'connection': <requests.adapters.HTTPAdapter object at 0x0000020BAA5454E0>}