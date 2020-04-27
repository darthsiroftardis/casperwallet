(function() {var implementors = {};
implementors["futures_channel"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"futures_channel/mpsc/struct.Receiver.html\" title=\"struct futures_channel::mpsc::Receiver\">Receiver</a>&lt;T&gt;","synthetic":false,"types":["futures_channel::mpsc::Receiver"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"futures_channel/mpsc/struct.UnboundedReceiver.html\" title=\"struct futures_channel::mpsc::UnboundedReceiver\">UnboundedReceiver</a>&lt;T&gt;","synthetic":false,"types":["futures_channel::mpsc::UnboundedReceiver"]}];
implementors["futures_core"] = [];
implementors["trust_dns_proto"] = [{"text":"impl&lt;S:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>&gt; <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"trust_dns_proto/tcp/struct.TcpClientStream.html\" title=\"struct trust_dns_proto::tcp::TcpClientStream\">TcpClientStream</a>&lt;S&gt;","synthetic":false,"types":["trust_dns_proto::tcp::tcp_client_stream::TcpClientStream"]},{"text":"impl&lt;S:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>&gt; <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"trust_dns_proto/tcp/struct.TcpStream.html\" title=\"struct trust_dns_proto::tcp::TcpStream\">TcpStream</a>&lt;S&gt;","synthetic":false,"types":["trust_dns_proto::tcp::tcp_stream::TcpStream"]},{"text":"impl&lt;S:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>, MF:&nbsp;<a class=\"trait\" href=\"trust_dns_proto/op/message/trait.MessageFinalizer.html\" title=\"trait trust_dns_proto::op::message::MessageFinalizer\">MessageFinalizer</a>&gt; <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"trust_dns_proto/udp/struct.UdpClientStream.html\" title=\"struct trust_dns_proto::udp::UdpClientStream\">UdpClientStream</a>&lt;S, MF&gt;","synthetic":false,"types":["trust_dns_proto::udp::udp_client_stream::UdpClientStream"]},{"text":"impl&lt;S:&nbsp;<a class=\"trait\" href=\"trust_dns_proto/udp/trait.UdpSocket.html\" title=\"trait trust_dns_proto::udp::UdpSocket\">UdpSocket</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static&gt; <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"trust_dns_proto/udp/struct.UdpStream.html\" title=\"struct trust_dns_proto::udp::UdpStream\">UdpStream</a>&lt;S&gt;","synthetic":false,"types":["trust_dns_proto::udp::udp_stream::UdpStream"]},{"text":"impl&lt;S, MF&gt; <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"trust_dns_proto/xfer/dns_multiplexer/struct.DnsMultiplexer.html\" title=\"struct trust_dns_proto::xfer::dns_multiplexer::DnsMultiplexer\">DnsMultiplexer</a>&lt;S, MF&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"trust_dns_proto/xfer/trait.DnsClientStream.html\" title=\"trait trust_dns_proto::xfer::DnsClientStream\">DnsClientStream</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;MF: <a class=\"trait\" href=\"trust_dns_proto/op/message/trait.MessageFinalizer.html\" title=\"trait trust_dns_proto::op::message::MessageFinalizer\">MessageFinalizer</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,&nbsp;</span>","synthetic":false,"types":["trust_dns_proto::xfer::dns_multiplexer::DnsMultiplexer"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()